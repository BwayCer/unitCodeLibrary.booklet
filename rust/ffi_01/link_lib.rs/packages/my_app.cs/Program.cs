using System.Text; // Encoding
using System.Runtime.InteropServices;
using LinkLib;
using FfiTest = LinkLib.Ffi;

class Program
{
    static void NormalOperation()
    {
        bool isTrace = true;
        bool isLeak = false;

        CallBaseFunc();
        CallPureData(isTrace);
        CallConcatString(isTrace, isLeak);
        CallConcatArray(isTrace, isLeak);
    }

    public static void TestMemoryLeak(bool isTrace, bool isLeak, long firstUsage)
    {
        string procFileName = GetProcFileName();

        const int sleepMs = 5000;
        const int callIntervalSleepMs = 700;
        int tryCount = 50000;
        int callIntervalCount = 2500;

        long lastUsage = ReadMemoryUsage(true, procFileName, 0, 0);

        CallBaseFunc();
        Console.WriteLine("---");
        lastUsage = ReadMemoryUsage(false, procFileName, lastUsage, firstUsage);
        ToSleep(3000);

        Console.WriteLine("\n---\ntest: call_pure_data");
        for (int i = 0; i < tryCount; i++)
        {
            CallPureData(isTrace);
            if (i % callIntervalCount == 0 && i != 0)
            {
                ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
                ToSleep(callIntervalSleepMs);
            }
        }
        lastUsage = ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
        ToSleep(sleepMs);

        Console.WriteLine("\n---\ntest: call_concat_string");
        for (int i = 0; i < tryCount; i++)
        {
            CallConcatString(isTrace, isLeak);
            if (i % callIntervalCount == 0 && i != 0)
            {
                ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
                ToSleep(callIntervalSleepMs);
            }
        }
        lastUsage = ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
        ToSleep(sleepMs);

        Console.WriteLine("\n---\ntest: call_concat_array");
        for (int i = 0; i < tryCount; i++)
        {
            CallConcatArray(isTrace, isLeak);
            if (i % callIntervalCount == 0 && i != 0)
            {
                ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
                ToSleep(callIntervalSleepMs);
            }
        }
        lastUsage = ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
        ToSleep(sleepMs);

        Console.WriteLine("\n---\ntest: wait 9 sec");
        for (int i = 0; i < 9; i++)
        {
            ToSleep(1000);
            ReadMemoryUsage(isTrace, procFileName, lastUsage, firstUsage);
        }
    }

    static string GetProcFileName()
    {
        int pid = System.Diagnostics.Process.GetCurrentProcess().Id;
        return $"/proc/{pid}/status";
    }

    static long ReadMemoryUsage(bool isTrace, string fileName, long prevUsage, long firstUsage)
    {
        // 強制執行垃圾回收
        GC.Collect();
        GC.WaitForPendingFinalizers();

        long value = -1;
        try
        {
            string[] lines = System.IO.File.ReadAllLines(fileName);
            foreach (string line in lines)
            {
                if (line.StartsWith("VmRSS:") || line.StartsWith("VmSize:"))
                {
                    string[] parts = line.Split(new char[] { ' ' }, StringSplitOptions.RemoveEmptyEntries);
                    if (parts.Length > 1 && long.TryParse(parts[1], out value))
                    {
                        break;
                    }
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"無法開啟檔案 {fileName}: {ex.Message}");
            return prevUsage;
        }

        if (value == -1)
        {
            Console.WriteLine("無法讀取記憶體使用量。");
            return prevUsage;
        }

        if (isTrace)
        {
            Console.WriteLine("---");
        }
        else
        {
            Console.Write("\r");
        }

        if (prevUsage == 0)
        {
            Console.WriteLine($"Memory usage: {value} KB");
        }
        else
        {
            Console.WriteLine(
                $"Memory usage: {value} KB (diff: {(value - prevUsage).ToString("+#;-#;0")} KB) (diff: {(value - firstUsage).ToString("+#;-#;0")} KB with first)"
            );
        }

        if (isTrace)
        {
            Console.WriteLine("---");
        }

        return value;
    }

    static void ToSleep(int timeMs)
    {
        Console.Write($"休眠 {timeMs} 毫秒...");
        Console.Out.Flush();
        Thread.Sleep(timeMs);
    }

    static void CallBaseFunc()
    {
        Console.WriteLine("Lang.cs: base_func: This function is called from a C# application.");
        FfiTest.base_func();
    }

    static void PrintPureDataArgs(PureDataArgs_t pureDataArgs)
    {
        Console.Write(
            $"PureDataArgs {{ {pureDataArgs.argu_16}, {pureDataArgs.argu_32}, {pureDataArgs.argu_64}, {(byte)pureDataArgs.argu_enum} }}"
        );
    }

    static void CallPureData(bool isTrace)
    {
        bool argu1 = true;
        sbyte argu2 = -8;
        PureDataArgs_t argu3 = new PureDataArgs_t
        {
            argu_64 = 6.4,
            argu_32 = -32,
            argu_16 = 16,
            argu_enum = FruitEnum_t.Apple
        };

        if (isTrace)
        {
            Console.Write(
                $"Lang.cs: pure_data: call: {argu1}, {argu2}, "
            );
            PrintPureDataArgs(argu3);
            Console.WriteLine();
        }

        PureDataResult_t result = FfiTest.pure_data(isTrace, argu1, argu2, argu3);

        if (isTrace)
        {
            Console.WriteLine(
                $"Lang.cs: pure_data: result: {result.rtn_yn}, {result.rtn_16}, {result.rtn_32}, {result.rtn_64}, {(byte)result.rtn_enum}"
            );
        }
    }

    static unsafe void CallConcatString(bool isTrace, bool isLeak)
    {
        string str1 = "Hello";
        string str2 = "World";
        if (isTrace)
        {
            Console.WriteLine($"Lang.cs: concat_string: call: {str1}, {str2}");
        }

        // byte* argu1 = (byte*)Marshal.StringToHGlobalAnsi(str1);
        // byte* argu2 = (byte*)Marshal.StringToHGlobalAnsi(str2);
        // try {...} finally {
        //     Marshal.FreeHGlobal((IntPtr)argu1);
        //     Marshal.FreeHGlobal((IntPtr)argu2);
        // }
        // or
        byte[] utf8Argu1 = Encoding.UTF8.GetBytes(str1);
        byte[] utf8Argu2 = Encoding.UTF8.GetBytes(str2);
        fixed (byte* argu1 = utf8Argu1)
        fixed (byte* argu2 = utf8Argu2)
        {
            if (argu1 == null || argu2 == null) {
                Console.WriteLine("Lang.cs: !!! Marshal.StringToHGlobalAnsi get null !!!");
            }

            byte* result = FfiTest.concat_string(isTrace, argu1, argu2);
            if (result != null)
            {
                int length = 0;
                while (result[length] != 0)
                {
                    length++;
                }
                string resultStr = Encoding.UTF8.GetString(result, length);

                if (isTrace)
                {
                    Console.WriteLine($"Lang.cs: concat_string: result: {resultStr}");
                }
                if (!isLeak)
                {
                    FfiTest.free_string(isTrace, result);
                }
            }
            else
            {
                Console.WriteLine("Lang.cs: concat_string: result: (null ptr)");
            }
        }
    }

    private static void PrintArrayRef(slice_ref_int32_t slice)
    {
        if (slice.len == 0)
        {
            Console.Write("[]");
            return;
        }
        unsafe
        {
            int* ptr = slice.ptr;
            Console.Write($"[{*ptr}");
            for (ulong i = 1; i < slice.len; ++i)
            {
                ptr++;
                Console.Write($", {*ptr}");
            }
            Console.Write("]");
        }
    }

    private static void PrintArrayBoxed(slice_boxed_int32_t slice)
    {
        if (slice.len == 0)
        {
            Console.Write("[]");
            return;
        }
        unsafe
        {
            int* ptr = slice.ptr;
            Console.Write($"[{*ptr}");
            for (ulong i = 1; i < slice.len; ++i)
            {
                ptr++;
                Console.Write($", {*ptr}");
            }
            Console.Write("]");
        }
    }

    public static unsafe void CallConcatArray(bool isTrace, bool isLeak)
    {
        int[] arr1 = { 4, 5, 6 };
        slice_ref_int32_t argu1;
        fixed (int* ptr1 = arr1)
        {
            argu1.ptr = ptr1;
            argu1.len = (UIntPtr)arr1.Length;
        }

        int[] arr2 = { 6, 7, 8, 9 };
        slice_ref_int32_t argu2;
        fixed (int* ptr2 = arr2)
        {
            argu2.ptr = ptr2;
            argu2.len = (UIntPtr)arr2.Length;
        }

        if (isTrace)
        {
            Console.Write("Lang.c: concat_array: call: ");
            PrintArrayRef(argu1);
            Console.Write(", ");
            PrintArrayRef(argu2);
            Console.WriteLine();
        }

        slice_boxed_int32_t result = FfiTest.concat_array(isTrace, argu1, argu2);

        if (isTrace)
        {
            Console.Write("Lang.c: concat_array: result: ");
            PrintArrayBoxed(result);
            Console.WriteLine();
        }

        if (!isLeak && result.ptr != null)
        {
            FfiTest.free_array_i32(isTrace, result);
        }
    }

    static void Main(string[] argv)
    {
        bool isNormalOperation = true;

        if (argv.Length > 0)
        {
            bool isTrace = argv.Contains("--trace");

            int tryRound = 2;
            {
                int index = Array.IndexOf(argv, "--try-round");
                if (index == -1)
                {
                    index = Array.IndexOf(argv, "-r");
                }
                if (index != -1 && index < argv.Length - 1)
                {
                    int num = 0;
                    if (int.TryParse(argv[index + 1], out int parsed))
                    {
                        num = parsed;
                    }
                    if (0 < num && num <= 100) {
                        tryRound = num;
                    }
                    else
                    {
                        Console.WriteLine($"`--try-round` 的參數應為數字類型並且介於 0..=100 之間. (actual: {argv[index + 1]})");
                        return;
                    }
                }
            }

            long firstUsage = ReadMemoryUsage(true, GetProcFileName(), 0, 0);
            if (argv.Contains("-l") || argv.Contains("--leak"))
            {
                isNormalOperation = false;
                TestMemoryLeak(true, true, firstUsage);
            }
            else if (argv.Contains("-t") || argv.Contains("--test"))
            {
                isNormalOperation = false;
                // 由於強制執行垃圾回收的效果不明顯, 改(人工)觀察同操作下漲幅是否相同.
                for (int i = 0; i < tryRound; i++)
                {
                    TestMemoryLeak(isTrace, false, firstUsage);
                }
            }
        }

        if (isNormalOperation)
        {
            NormalOperation();
        }
    }
}
