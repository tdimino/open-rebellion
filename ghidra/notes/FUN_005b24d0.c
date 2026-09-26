
void __thiscall FUN_005b24d0(void *this,uint param_1)

{
  void *this_00;
  int iVar1;
  uint uVar2;
  int *piVar3;
  
  uVar2 = param_1;
  this_00 = (void *)FUN_00596bd0();
  iVar1 = FUN_005a6340(this_00,uVar2);
  uVar2 = 0;
  piVar3 = (int *)((int)this + 900);
  do {
    if (*piVar3 == 0) break;
    uVar2 = uVar2 + 1;
    piVar3 = piVar3 + 1;
  } while (uVar2 < 4);
  if (uVar2 < 4) {
    *(uint *)((int)this + uVar2 * 4 + 900) = param_1;
    *(undefined4 *)((int)this + 0x3a8) = 0;
    FUN_005b2520(iVar1);
  }
  return;
}

