
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_005b2720(int param_1)

{
  void *this;
  int *piVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  
  uVar3 = **(uint **)(param_1 + 4);
  this = (void *)FUN_00596bd0();
  piVar1 = (int *)FUN_005a6340(this,uVar3);
  if (piVar1 != (int *)0x0) {
    piVar4 = piVar1;
    FUN_005aaf90();
    iVar2 = FUN_005ad870(piVar4);
    if ((iVar2 != 0) && (_DAT_0066c4f0 < (float)piVar1[0xf3])) {
      (**(code **)(*piVar1 + 0xd0))();
      piVar1[0x3d] = 0;
      piVar1[0x40] = 0;
      piVar1[0x44] = 0;
      uVar3 = piVar1[0x3b];
      iVar2 = (**(code **)(*piVar1 + 0xb8))();
      FUN_005c13d0(uVar3,4,iVar2);
      piVar1[0x5c] = 2;
      (**(code **)(*piVar1 + 0x1c))();
    }
  }
  return;
}

