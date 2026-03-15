
void __thiscall FUN_0042cde0(void *this,uint param_1)

{
  void *pvVar1;
  uint *puVar2;
  int iVar3;
  BOOL BVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  undefined4 *puVar9;
  uint uVar10;
  tagRECT local_3c;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c778;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  SetRectEmpty(&local_3c);
  FUN_00512f90(auStack_2c,param_1,4);
  iStack_4 = 0;
  FUN_00513120((int)auStack_2c);
  if (iStack_10 != 0) {
    do {
      pvVar1 = (void *)FUN_0052bed0((int)auStack_2c);
      puVar2 = FUN_004025b0(pvVar1,&param_1);
      iStack_4._0_1_ = 1;
      iVar3 = FUN_0060a860((void *)((int)this + 0x484),*puVar2 & 0xffffff);
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      FUN_00619730();
      if (iVar3 != 0) {
        iVar6 = *(int *)(iVar3 + 0x2c);
        iVar5 = *(int *)(iVar3 + 0x28);
        pvVar1 = *(void **)(iVar3 + 0x24);
        uVar10 = 0;
        puVar9 = (undefined4 *)0x0;
        uVar8 = 0;
        iVar7 = 0;
        puVar2 = (uint *)FUN_005ff440((int)this);
        FUN_005fcc30(pvVar1,puVar2,iVar5,iVar6,iVar7,uVar8,puVar9,uVar10);
        UnionRect(&local_3c,&local_3c,(RECT *)(iVar3 + 0x40));
      }
      FUN_005130d0((int)auStack_2c);
    } while (iStack_10 != 0);
  }
  FUN_00513120((int)auStack_2c);
  if (iStack_10 != 0) {
    do {
      pvVar1 = (void *)FUN_0052bed0((int)auStack_2c);
      puVar2 = FUN_004025b0(pvVar1,&param_1);
      iStack_4._0_1_ = 2;
      iVar3 = FUN_0060a860((void *)((int)this + 0x484),*puVar2 & 0xffffff);
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      FUN_00619730();
      if (iVar3 != 0) {
        if ((*(uint *)(iVar3 + 0x3c) & 0x80) == 0) {
          if ((*(uint *)(iVar3 + 0x3c) & 8) != 0) {
            iVar6 = *(int *)(iVar3 + 0x2c);
            iVar5 = *(int *)(iVar3 + 0x28);
            puVar2 = (uint *)FUN_005ff440((int)this);
            FUN_005fd0f0(*(void **)((int)this + 0x2c8),puVar2,iVar5,iVar6);
          }
          pvVar1 = *(void **)(iVar3 + 0x20);
          if (pvVar1 == (void *)0x0) goto LAB_0042cf78;
          iVar6 = *(int *)(iVar3 + 0x2c);
          iVar3 = *(int *)(iVar3 + 0x28);
          puVar2 = (uint *)FUN_005ff440((int)this);
        }
        else {
          iVar6 = *(int *)(iVar3 + 0x2c);
          iVar3 = *(int *)(iVar3 + 0x28);
          puVar2 = (uint *)FUN_005ff440((int)this);
          pvVar1 = *(void **)((int)this + 0x2c4);
        }
        FUN_005fd0f0(pvVar1,puVar2,iVar3,iVar6);
      }
LAB_0042cf78:
      FUN_005130d0((int)auStack_2c);
    } while (iStack_10 != 0);
  }
  BVar4 = IsRectEmpty(&local_3c);
  if (BVar4 == 0) {
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_3c,0);
  }
  iStack_4 = 0xffffffff;
  FUN_00513010(auStack_2c);
  ExceptionList = pvStack_c;
  return;
}

