
uint __thiscall FUN_00537ff0(void *this,int *param_1)

{
  int iVar1;
  uint uVar2;
  void *pvVar3;
  uint *puVar4;
  int *piVar5;
  uint uVar6;
  bool bVar7;
  bool bVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  int iStack_40;
  undefined1 auStack_3c [4];
  undefined1 auStack_38 [4];
  undefined4 uStack_34;
  int iStack_30;
  undefined4 auStack_2c [2];
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 auStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00645e18;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_1 + 4))();
  FUN_00541e70(auStack_1c);
  uVar6 = 0;
  iStack_4 = 0;
  FUN_0051fcb0(this,auStack_1c);
  FUN_004f4340(auStack_2c);
  uStack_34 = 1;
  iStack_30 = -1;
  puVar10 = &uStack_34;
  puVar9 = auStack_2c;
  iStack_4._0_1_ = 2;
  iVar1 = (**(code **)(*(int *)this + 0x28))();
  uVar2 = FUN_0053f150(auStack_1c,iVar1,puVar9,puVar10);
  if ((uVar2 != 0) && (iStack_30 == -1)) {
    iStack_40 = 0;
    bVar7 = true;
    pvVar3 = (void *)thunk_FUN_005f5060((int)auStack_2c);
    do {
      if (pvVar3 == (void *)0x0) break;
      puVar4 = FUN_00403040(pvVar3,auStack_3c);
      uStack_24 = 0xa0;
      uStack_20 = 0xb0;
      iStack_4._0_1_ = 3;
      if ((*puVar4 >> 0x18 < 0xa0) || (0xaf < *puVar4 >> 0x18)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
      FUN_00619730();
      iStack_4._0_1_ = 2;
      FUN_00619730();
      if (bVar8) {
        puVar4 = FUN_00403040(pvVar3,auStack_38);
        iStack_4._0_1_ = 4;
        piVar5 = (int *)FUN_00505750(puVar4);
        iStack_4._0_1_ = 2;
        FUN_00619730();
        uVar2 = (uint)(piVar5 != (int *)0x0);
        if ((uVar2 != 0) && (bVar7)) {
          uVar6 = (uint)piVar5[9] >> 6 & 3;
          iStack_40 = (**(code **)(*piVar5 + 0x1ec))();
          bVar7 = false;
        }
      }
      pvVar3 = *(void **)((int)pvVar3 + 0x10);
    } while (uVar2 != 0);
    bVar8 = uVar2 != 0;
    uVar2 = 0;
    if (((bVar8) && (uVar2 = (uint)!bVar7, uVar2 != 0)) && (uVar6 != 3)) {
      if (uVar6 == 1) {
        iVar1 = 1;
      }
      else if (uVar6 == 2) {
        iVar1 = 2;
      }
      else {
        iVar1 = 0;
      }
      pvVar3 = (void *)FUN_00506f30(iVar1);
      uVar6 = 0;
      bVar7 = true;
      if (iStack_40 == 0) {
        uVar6 = 0x28;
      }
      else if (iStack_40 == 1) {
        uVar6 = 0x2a;
      }
      else if (iStack_40 == 2) {
        uVar6 = 0x29;
      }
      else {
        bVar7 = false;
      }
      uVar2 = 0;
      if (bVar7) {
        uVar2 = FUN_0052e580(pvVar3,uVar6,param_1);
      }
    }
  }
  iStack_4._0_1_ = 1;
  FUN_00619730();
  iStack_4 = (uint)iStack_4._1_3_ << 8;
  FUN_004f4380(auStack_2c);
  iStack_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return uVar2;
}

