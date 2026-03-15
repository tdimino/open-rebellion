
undefined4 __fastcall FUN_00599bc0(int param_1)

{
  undefined *puVar1;
  int iVar2;
  
  iVar2 = *(int *)(param_1 + 0x690) + 1;
  *(int *)(param_1 + 0x690) = iVar2;
  if (iVar2 < 0) {
    *(undefined4 *)(param_1 + 0x690) = 0;
  }
  puVar1 = DAT_006bc4a8;
  if (DAT_006bc4a8 != (undefined *)0x0) {
    FUN_005afa50((int)DAT_006bc4a8);
    FUN_00618b60(puVar1);
  }
  DAT_006bc4a8 = (undefined *)0x0;
  FUN_005aafa0();
  FUN_0059ede0();
  FUN_005bbee0();
  FUN_00401ad0();
  iVar2 = *(int *)(param_1 + 0x690) + -1;
  *(int *)(param_1 + 0x690) = iVar2;
  if (iVar2 < 0) {
    *(undefined4 *)(param_1 + 0x690) = 0;
  }
  return 1;
}

