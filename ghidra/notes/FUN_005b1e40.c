
void __cdecl FUN_005b1e40(int param_1)

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
    if (iVar2 != 0) {
      iVar2 = FUN_005b1b70(piVar1);
      if (iVar2 == 1) {
        piVar1[0x5c] = 7;
        return;
      }
      FUN_005b2720(param_1);
    }
  }
  return;
}

