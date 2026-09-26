
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005ba7f0(void *this,float param_1)

{
  bool bVar1;
  float fVar2;
  int iVar3;
  void *this_00;
  uint uVar4;
  
  if ((_DAT_0066c710 < *(float *)((int)this + 0x68)) && (*(int *)((int)this + 0x74) == 0)) {
    fVar2 = param_1 * _DAT_0066c708 * _DAT_0066c718;
    if ((*(float *)((int)this + 0x6c) < _DAT_0066c71c) &&
       (_DAT_0066c71c <= fVar2 + *(float *)((int)this + 0x6c))) {
      iVar3 = FUN_005aaf90();
      iVar3 = *(int *)(iVar3 + 0x9ec);
      if (iVar3 == 1) {
        uVar4 = 0x113;
        this_00 = (void *)FUN_005ba8b0();
        FUN_005ba980(this_00,uVar4,iVar3);
      }
    }
    fVar2 = fVar2 + *(float *)((int)this + 0x6c);
    bVar1 = _DAT_0066c71c <= fVar2;
    *(float *)((int)this + 0x6c) = fVar2;
    if (bVar1) {
      fVar2 = _DAT_0066c71c;
    }
    *(float *)((int)this + 0x6c) = fVar2;
  }
  return;
}

