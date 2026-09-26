
int * __thiscall FUN_00474c00(void *this,undefined4 *param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  int *piVar6;
  undefined4 uStack_20;
  
  piVar6 = (int *)0x0;
  uVar4 = *(uint *)((int)this + 0x2c) >> 0x18;
  bVar2 = true;
  bVar3 = true;
  if ((uVar4 < 8) || (0xf < uVar4)) {
    bVar2 = false;
  }
  uStack_20 = 0x474c42;
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)((int)this + 0x4c);
    FUN_004f26d0(&uStack_20,(undefined4 *)((int)this + 0x2c));
    pvVar5 = FUN_00419e40((void *)(iVar1 + 0x78));
    if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x38) & 2) != 0)) {
      bVar3 = false;
    }
  }
  if (bVar3) {
    if (*(int *)((int)this + 0x34) < 0) {
      uStack_20 = 0x474c86;
      piVar6 = FUN_00475490(this);
      if (piVar6 != (int *)0x0) {
        return piVar6;
      }
      uStack_20 = 0x474c93;
      piVar6 = FUN_00475890(this);
    }
    if (piVar6 == (int *)0x0) {
      if (0 < *(int *)((int)this + 0x34)) {
        uStack_20 = 0x474ca7;
        piVar6 = FUN_00475c70(this);
        *param_1 = 1;
        return piVar6;
      }
      *param_1 = 1;
    }
  }
  return piVar6;
}

