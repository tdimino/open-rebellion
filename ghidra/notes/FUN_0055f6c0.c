
int __thiscall FUN_0055f6c0(int *param_1,undefined4 param_2,undefined4 param_3)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar1 = FUN_0053a000();
  if (iVar1 != 0) {
    iVar2 = FUN_0053a640(2,param_2,param_1 + 0x16);
    if (iVar2 != 0) {
      uVar3 = param_3;
      FUN_00539fd0(1);
      FUN_00562370(uVar3);
      uVar3 = param_3;
      FUN_00539fd0(2);
      FUN_00562370(uVar3);
      (**(code **)(*param_1 + 0x1dc))(param_2,param_3);
    }
  }
  return iVar1;
}

