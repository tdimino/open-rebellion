
undefined4 FUN_00561b60(int *param_1,undefined4 param_2)

{
  int iVar1;
  int iVar2;
  undefined4 local_4;
  
  local_4 = 0;
  iVar1 = FUN_0055e810(&local_4);
  iVar2 = (**(code **)(*param_1 + 0x2f0))(local_4,10,param_2);
  if ((iVar2 != 0) && (iVar1 != 0)) {
    return 1;
  }
  return 0;
}

