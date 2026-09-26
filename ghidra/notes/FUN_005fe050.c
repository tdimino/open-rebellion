
void __thiscall FUN_005fe050(void *this,short param_1,short param_2,short param_3,short param_4)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  
  uVar1 = FUN_005fc9c0(this);
  iVar2 = FUN_005fc110(this);
  if ((iVar2 == 0) && (*(int *)((int)this + 0x24) != 0)) {
    iVar2 = FUN_005fc0f0(this);
    if (iVar2 < 1) {
      iVar2 = -iVar2;
    }
    iVar4 = (int)param_2;
    if (iVar2 < param_4 + iVar4) {
      param_4 = ((short)iVar2 - param_2) + 1;
    }
    iVar2 = FUN_005fc0e0(this);
    if (iVar2 < (int)param_3 + (int)param_1) {
      iVar2 = FUN_005fc0e0(this);
      param_3 = (short)iVar2 + (1 - param_1);
    }
    if (((0 < param_4) && (0 < param_3)) && (_param_2 = (int)param_4, 0 < _param_2)) {
      do {
        iVar2 = FUN_005fc9e0(this,iVar4);
        iVar5 = uVar1 * iVar2 + *(int *)((int)this + 4) + (int)param_1;
        iVar2 = 0;
        if (0 < param_3) {
          do {
            iVar3 = iVar2 + 1;
            *(undefined1 *)(iVar2 + iVar5) =
                 *(undefined1 *)(*(int *)((int)this + 0x24) + (uint)*(byte *)(iVar2 + iVar5));
            iVar2 = iVar3;
          } while (iVar3 < param_3);
        }
        iVar4 = iVar4 + 1;
        _param_2 = _param_2 + -1;
      } while (_param_2 != 0);
    }
  }
  return;
}

