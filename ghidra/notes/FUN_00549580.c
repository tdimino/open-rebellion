
int FUN_00549580(uint param_1,void *param_2)

{
  void *pvVar1;
  undefined1 uVar2;
  bool bVar3;
  void *this;
  uint *puVar4;
  int *this_00;
  int iVar5;
  uint uVar6;
  undefined4 uVar7;
  undefined3 extraout_var;
  int iVar8;
  void *pvVar9;
  undefined4 uStack_2c;
  void *local_28;
  undefined1 local_24 [4];
  uint local_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647c60;
  local_c = ExceptionList;
  iVar8 = 1;
  ExceptionList = &local_c;
  this = (void *)thunk_FUN_005f5060(param_1);
  pvVar1 = param_2;
  for (; this != (void *)0x0; this = *(void **)((int)this + 0x10)) {
    puVar4 = FUN_00403040(this,local_24);
    local_4 = 0;
    this_00 = (int *)FUN_00505d40(puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((this_00 == (int *)0x0) || (iVar8 == 0)) {
      iVar8 = 0;
    }
    else {
      iVar8 = 1;
    }
    if (this_00 != (int *)0x0) {
      FUN_004f4340(local_14);
      local_4 = 1;
      puVar4 = FUN_004025b0(this_00,&local_20);
      local_4._0_1_ = 2;
      FUN_004f44b0(local_14,puVar4,0);
      local_4._0_1_ = 1;
      FUN_00619730();
      if ((*(byte *)(this_00 + 0x14) & 8) == 0) {
        iVar5 = (**(code **)(*this_00 + 0x1d4))();
        if (iVar5 == 0) {
          uVar7 = FUN_004ece30(&uStack_1c);
          local_4._0_1_ = 3;
          iVar5 = (**(code **)(*this_00 + 0x20c))(uVar7,pvVar1);
          if ((iVar5 == 0) || (iVar8 == 0)) {
            iVar8 = 0;
          }
          else {
            iVar8 = 1;
          }
          local_4._0_1_ = 1;
          FUN_00619730();
        }
        else {
          iVar5 = (**(code **)(*this_00 + 0x218))(pvVar1);
          if ((iVar5 == 0) || (iVar8 == 0)) {
            iVar8 = 0;
          }
          else {
            iVar8 = 1;
          }
        }
      }
      FUN_004ece30(&param_1);
      local_4._0_1_ = 4;
      uVar2 = (undefined1)local_4;
      local_4._0_1_ = 4;
      if ((*(byte *)(this_00 + 0x14) & 8) == 0) {
        uVar6 = FUN_00543040(this_00,(uint *)0x0,&param_1);
        if ((uVar6 == 0) || (iVar8 == 0)) {
          iVar8 = 0;
        }
        else {
          iVar8 = 1;
        }
        puVar4 = FUN_004ece40(&param_1);
        uVar2 = (undefined1)local_4;
        if (puVar4 == (uint *)0x0) {
          if (((*(byte *)(this_00 + 0x14) & 8) == 0) &&
             (iVar5 = (**(code **)(*this_00 + 0x1d4))(), iVar5 == 0)) {
            uVar7 = FUN_004ece30(&uStack_18);
            local_4._0_1_ = 5;
            iVar5 = (**(code **)(*this_00 + 0x20c))(uVar7,pvVar1);
            if ((iVar5 == 0) || (iVar8 == 0)) {
              iVar8 = 0;
            }
            else {
              iVar8 = 1;
            }
            local_4._0_1_ = 4;
            FUN_00619730();
          }
          if (((byte)this_00[9] & 0xc0) == 0x40) {
            param_2 = (void *)0x98000440;
            local_4._0_1_ = 6;
            FUN_004f26d0(&param_1,&param_2);
          }
          else {
            uStack_2c = 0x98000480;
            local_4._0_1_ = 7;
            FUN_004f26d0(&param_1,&uStack_2c);
          }
          local_4._0_1_ = 4;
          FUN_00619730();
          uVar2 = (undefined1)local_4;
        }
      }
      local_4._0_1_ = uVar2;
      iVar5 = FUN_004f78e0(this_00,0,pvVar1);
      if ((iVar5 == 0) || (iVar8 == 0)) {
        iVar8 = 0;
      }
      else {
        iVar8 = 1;
      }
      if ((*(byte *)(this_00 + 0x14) & 8) == 0) {
        uVar6 = this_00[9];
        pvVar9 = pvVar1;
        iVar5 = (**(code **)(*this_00 + 0x1d4))();
        bVar3 = FUN_00547740(local_28,uVar6 >> 6 & 3,&param_1,(int)local_14,iVar5,(uint)pvVar9);
        if ((CONCAT31(extraout_var,bVar3) == 0) || (iVar8 == 0)) {
          iVar8 = 0;
        }
        else {
          iVar8 = 1;
        }
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_004f4380(local_14);
    }
  }
  ExceptionList = local_c;
  return iVar8;
}

