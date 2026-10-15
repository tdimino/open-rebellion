
undefined4 __fastcall FUN_00500f60(int *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  uint uVar4;
  
  uVar3 = 0;
  iVar1 = FUN_00500670();
  if (iVar1 != 0) {
    uVar4 = (uint)param_1[0x19] >> 4 & 0xf;
    iVar1 = FUN_00500670();
    iVar2 = (**(code **)(*param_1 + 0x264))();
    uVar3 = FUN_0053e170(iVar2,uVar4,iVar1);
  }
  return uVar3;
}

