
int FUN_0053cbc0(void *param_1,int *param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  void *this;
  uint *puVar3;
  int *piVar4;
  uint uVar5;
  int iVar6;
  int iStack_3c;
  int iStack_38;
  undefined1 auStack_34 [4];
  undefined1 auStack_30 [4];
  undefined1 auStack_2c [4];
  void *pvStack_28;
  uint uStack_24;
  uint uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00646588;
  pvStack_c = ExceptionList;
  iVar6 = 1;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_2 + 4))();
  uStack_24 = 1;
  uStack_20 = 0xffffffff;
  uStack_4 = 0;
  FUN_00520580(param_3,&uStack_24);
  uStack_4 = 0xffffffff;
  FUN_00619730();
  FUN_004ece30(&iStack_38);
  uStack_4 = 1;
  FUN_004ece30(&iStack_3c);
  uStack_4._0_1_ = 2;
  bVar2 = false;
  this = (void *)thunk_FUN_005f5060((int)param_1);
  do {
    if ((*(int *)((int)param_3 + 4) != -1) || (this == (void *)0x0)) break;
    pvStack_28 = *(void **)((int)this + 0x10);
    puVar3 = FUN_00403040(this,auStack_34);
    uStack_14 = 0x30;
    uStack_10 = 0x40;
    uStack_4._0_1_ = 3;
    if ((*puVar3 >> 0x18 < 0x30) || (0x3f < *puVar3 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    uStack_4._0_1_ = 2;
    FUN_00619730();
    if (bVar1) {
      puVar3 = FUN_00403040(this,auStack_30);
      uStack_4._0_1_ = 4;
      piVar4 = (int *)FUN_00504dc0(puVar3);
      uStack_4._0_1_ = 2;
      FUN_00619730();
      if (piVar4 == (int *)0x0) {
        uStack_1c = 1;
        uStack_18 = 0x12;
        uStack_4._0_1_ = 5;
        FUN_00520580(param_3,&uStack_1c);
        uStack_4._0_1_ = 2;
        FUN_00619730();
      }
      if (iVar6 != 0) {
        if (*(int *)((int)param_3 + 4) == -1) {
          uStack_24 = 0x30;
          uStack_20 = 0x40;
          uStack_4._0_1_ = 6;
          uVar5 = (**(code **)(*piVar4 + 4))();
          if ((uVar5 < uStack_24) || (uStack_20 <= uVar5)) {
            iVar6 = 0;
          }
          else {
            iVar6 = 1;
          }
          uStack_4._0_1_ = 2;
          FUN_00619730();
        }
        if ((iVar6 != 0) && ((*(byte *)(piVar4 + 0x14) & 4) != 0)) {
          if (*(int *)((int)param_3 + 4) == -1) {
            iVar6 = (**(code **)(*piVar4 + 0xc))(&iStack_3c);
          }
          if ((iVar6 != 0) && (*(int *)((int)param_3 + 4) == -1)) {
            if (!bVar2) {
              bVar2 = true;
              FUN_004f26d0(&iStack_38,&iStack_3c);
            }
            if (iStack_38 == iStack_3c) {
              puVar3 = FUN_00403040(this,auStack_2c);
              uStack_4._0_1_ = 7;
              FUN_004f5910(param_1,puVar3);
              uStack_4._0_1_ = 2;
              FUN_00619730();
              FUN_004f57b0(param_2,this);
            }
          }
        }
      }
    }
    this = pvStack_28;
  } while (iVar6 != 0);
  if (iVar6 == 0) {
    uStack_1c = 1;
    uStack_18 = 1;
    uStack_4._0_1_ = 8;
    FUN_00520580(param_3,&uStack_1c);
    uStack_4._0_1_ = 2;
    FUN_00619730();
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_00619730();
  uStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar6;
}

