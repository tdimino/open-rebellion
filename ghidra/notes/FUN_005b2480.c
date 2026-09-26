
void __thiscall FUN_005b2480(void *this,uint param_1)

{
  void *this_00;
  int iVar1;
  uint uVar2;
  uint *puVar3;
  
  uVar2 = param_1;
  this_00 = (void *)FUN_00596bd0();
  iVar1 = FUN_005a6340(this_00,uVar2);
  uVar2 = 0;
  puVar3 = (uint *)((int)this + 900);
  do {
    if (*puVar3 == param_1) break;
    uVar2 = uVar2 + 1;
    puVar3 = puVar3 + 1;
  } while (uVar2 < 4);
  if (uVar2 < 4) {
    *(undefined4 *)((int)this + uVar2 * 4 + 900) = 0;
    *(undefined4 *)((int)this + 0x3a8) = 0;
    FUN_005b2550(iVar1);
  }
  return;
}

