
undefined4 __fastcall FUN_00500f00(int *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  
  iVar1 = (**(code **)(*param_1 + 0x248))();
  iVar2 = (**(code **)(*param_1 + 0x274))();
  iVar3 = (**(code **)(*param_1 + 0x25c))();
  uVar4 = FUN_0053e170(iVar3,iVar2,iVar1);
  if ((param_1[9] & 0x30U) == 0) {
    iVar1 = FUN_005006e0((int)param_1);
    if (iVar1 != 0) {
      uVar4 = (**(code **)(*param_1 + 0x25c))();
    }
  }
  return uVar4;
}

