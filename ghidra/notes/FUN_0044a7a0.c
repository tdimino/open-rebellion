
void FUN_0044a7a0(int *param_1,void *param_2,uint *param_3)

{
  bool bVar1;
  uint *puVar2;
  uint uVar3;
  undefined4 *puVar4;
  int iVar5;
  void *this;
  uint uStack_20;
  int local_1c;
  undefined4 *puStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062f376;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_1 + 4))();
  FUN_004ece30(&uStack_20);
  this = param_2;
  uStack_4 = 0;
  if (param_2 != (void *)0x0) {
    puVar2 = FUN_004025b0(param_2,(uint *)&param_2);
    uStack_4._0_1_ = 1;
    FUN_004f26d0(&uStack_20,puVar2);
    uStack_4._0_1_ = 0;
    FUN_00619730();
    do {
      if ((*(byte *)((int)this + 0x50) & 0x40) != 0) {
        iVar5 = *(int *)(*(int *)(local_1c + 0x114) + 0x9c);
        if (iVar5 == 1) {
          uVar3 = 1;
        }
        else if (iVar5 == 2) {
          uVar3 = 2;
        }
        else {
          uVar3 = 0;
        }
        if (uVar3 == (*(uint *)((int)this + 0x24) >> 6 & 3)) {
          puVar2 = FUN_004025b0(this,(uint *)&param_2);
          uStack_4._0_1_ = 2;
          FUN_004f26d0(&uStack_20,puVar2);
          uStack_4._0_1_ = 0;
          FUN_00619730();
          puVar4 = (undefined4 *)
                   FUN_005f5500(param_1,*(uint *)(*(int *)((int)this + 0x2c) + 0x30) & 0xffff);
          if (puVar4 == (undefined4 *)0x0) {
            puVar4 = (undefined4 *)FUN_00618b70(0x28);
            uStack_4._0_1_ = 3;
            puStack_18 = puVar4;
            if (puVar4 == (undefined4 *)0x0) {
              puVar4 = (undefined4 *)0x0;
            }
            else {
              FUN_005f5590(puVar4);
              uStack_4._0_1_ = 4;
              FUN_004ece30(puVar4 + 9);
              uStack_4._0_1_ = 5;
              *puVar4 = &PTR_FUN_00659c70;
              puVar4[7] = 0;
              puVar4[8] = 0;
              FUN_004f26d0(puVar4 + 9,&uStack_20);
            }
            uStack_4._0_1_ = 0;
            FUN_005f5600(puVar4,*(uint *)(*(int *)((int)this + 0x2c) + 0x30) & 0xffff);
            FUN_005f5440(param_1,puVar4);
          }
          uStack_14 = 0x10;
          uStack_10 = 0x40;
          puVar4[8] = puVar4[8] + 1;
          if ((uStack_20 >> 0x18 < 0x10) || (0x3f < uStack_20 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            iVar5 = FUN_004f2990((int)this);
            puVar4[7] = puVar4[7] + iVar5;
          }
        }
      }
      this = (void *)FUN_004f6010(this,param_3,1);
    } while (this != (void *)0x0);
    uStack_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = pvStack_c;
    return;
  }
  uStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

