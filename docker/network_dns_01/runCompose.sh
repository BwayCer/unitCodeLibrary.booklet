#/bin/env bash


echo "# 列出網路"
cmdList=(docker network ls)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 列出容器的網路"
cmdList=(docker ps --format "table {{.ID}}\t{{.Names}}\t{{.Networks}}")
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e '\n\n# 顯示 c02 容器的 "/etc/hosts" 文件'
cmdList=(docker exec bway_c02 cat /etc/hosts)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 從 c02 容器 ping c01 容器"
cmdList=(docker exec bway_c02 ping c1 -c 2)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 從 c03 容器 ping c01 容器"
cmdList=(docker exec bway_c03 ping c1 -c 2)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


exit 0
