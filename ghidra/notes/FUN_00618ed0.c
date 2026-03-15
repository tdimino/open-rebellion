
int __cdecl FUN_00618ed0(undefined1 *param_1,char *param_2)

{
  int iVar1;
  undefined1 *local_20;
  int local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  
  local_18 = param_1;
  local_20 = param_1;
  local_14 = 0x42;
  local_1c = 0x7fffffff;
  iVar1 = FUN_0061e470((int *)&local_20,param_2,(undefined4 *)&stack0x0000000c);
  local_1c = local_1c + -1;
  if (-1 < local_1c) {
    *local_20 = 0;
    return iVar1;
  }
  FUN_0061e340(0,(int *)&local_20);
  return iVar1;
}

