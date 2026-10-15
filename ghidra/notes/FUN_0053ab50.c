
int __thiscall FUN_0053ab50(int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  int iVar2;
  
  uVar1 = param_2;
  iVar2 = FUN_00569840(param_2);
  if (iVar2 != 0) {
    FUN_005f4d90(uVar1,&param_2);
    *(undefined4 *)(param_1 + 0x58) = param_2;
    FUN_005f4d90(uVar1,param_1 + 0x5c);
    FUN_005f4d90(uVar1,param_1 + 0x60);
  }
  return iVar2;
}

