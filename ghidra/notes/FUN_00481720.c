
undefined4 __fastcall FUN_00481720(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  void *pvVar5;
  undefined4 *puVar6;
  uint local_24;
  undefined4 local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634200;
  local_c = ExceptionList;
  bVar2 = false;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  FUN_004ece80((undefined4 *)(param_1 + 0x30));
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffef;
  if ((*(byte *)(param_1 + 0x20) & 1) == 0) {
    uVar3 = *(uint *)(param_1 + 0x34) >> 0x18;
    local_14 = 0x90;
    local_10 = 0x98;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(param_1 + 0x34));
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar4 != (void *)0x0) {
        *(uint *)((int)pvVar4 + 0x2c) = *(uint *)((int)pvVar4 + 0x2c) & 0xdfffffff;
      }
    }
    FUN_004ece80((undefined4 *)(param_1 + 0x34));
LAB_00481e79:
    if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) goto LAB_00481e87;
  }
  else {
    this = (uint *)(param_1 + 0x34);
    uVar3 = *(uint *)(param_1 + 0x34) >> 0x18;
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((0x8f < uVar3) && (uVar3 < 0x98)) {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,this);
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar4 == (void *)0x0) goto LAB_00481801;
      if (((*(byte *)((int)pvVar4 + 0x2c) & 0x40) != 0) &&
         ((*(uint *)((int)pvVar4 + 0x30) & 0x200) == 0)) {
        FUN_004f26d0((undefined4 *)(param_1 + 0x30),this);
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
        *(uint *)((int)pvVar4 + 0x2c) = *(uint *)((int)pvVar4 + 0x2c) | 0x20000000;
      }
      if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
        *(uint *)((int)pvVar4 + 0x2c) = *(uint *)((int)pvVar4 + 0x2c) & 0xdfffffff;
        goto LAB_00481801;
      }
    }
    else {
LAB_00481801:
      if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
        FUN_004ece80(this);
        local_1c = (undefined1 *)0x90;
        local_18 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x68),0,0x4000020,4,0,0,0,7,&DAT_00000002);
          pvVar4 = (void *)(param_1 + 0x54);
          FUN_00435790(pvVar4,(int)pvVar5);
          puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
          local_4._0_1_ = 1;
          FUN_004f26d0(&local_24,puVar6);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar4);
          local_1c = (undefined1 *)0x80;
          local_18 = 0x90;
          if ((local_24 >> 0x18 < 0x80) || (0x8f < local_24 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) {
            pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0x68) + 0x44),&local_24);
            if ((pvVar5 != (void *)0x0) && (*(int *)((int)pvVar5 + 0x8c) < 0x46)) {
              pvVar5 = FUN_0041a460(*(void **)(param_1 + 0x68),&local_24,0,0x40,1,0,0x20000000,0,
                                    &DAT_00000002);
              FUN_00435790(pvVar4,(int)pvVar5);
              puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
              local_4._0_1_ = 2;
              FUN_004f26d0(this,puVar6);
              local_4._0_1_ = 0;
              FUN_00619730();
              FUN_005f58b0(pvVar4);
            }
          }
        }
        local_1c = (undefined1 *)0x90;
        local_18 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          local_18 = 0x98;
          local_1c = (undefined1 *)0x90;
          uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
          if ((uVar3 < 0x90) || (0x97 < uVar3)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) {
            puVar6 = FUN_00419b80(*(void **)(param_1 + 0x68),(undefined4 *)(param_1 + 0x38),0,
                                  0x4000020,0x10,0,0,0,2);
            pvVar4 = (void *)(param_1 + 0x54);
            FUN_00435790(pvVar4,(int)puVar6);
            puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
            local_4._0_1_ = 3;
            FUN_004f26d0(&local_24,puVar6);
            local_4._0_1_ = 0;
            FUN_00619730();
            FUN_005f58b0(pvVar4);
            pvVar5 = FUN_0041a460(*(void **)(param_1 + 0x68),&local_24,0,0x40,2,0,0x20000000,0,
                                  &DAT_00000002);
            FUN_00435790(pvVar4,(int)pvVar5);
            puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
            local_4._0_1_ = 4;
            FUN_004f26d0(this,puVar6);
            local_4._0_1_ = 0;
            FUN_00619730();
            FUN_005f58b0(pvVar4);
          }
        }
        local_1c = (undefined1 *)0x90;
        local_18 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
          local_1c = (undefined1 *)0x90;
          local_18 = 0x98;
          if ((uVar3 < 0x90) || (0x97 < uVar3)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) {
            puVar6 = FUN_0041a400(*(void **)(param_1 + 0x68),&local_24,
                                  (undefined4 *)(param_1 + 0x38),0,0x40,0,0,0x20000000,0,
                                  (undefined4 *)&DAT_00000002);
            pvVar4 = (void *)(param_1 + 0x54);
            FUN_00435790(pvVar4,(int)puVar6);
            puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
            local_4._0_1_ = 5;
            FUN_004f26d0(this,puVar6);
            local_4._0_1_ = 0;
            FUN_00619730();
            FUN_005f58b0(pvVar4);
          }
        }
        local_1c = (undefined1 *)0x90;
        local_18 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x68),0,0x4000020,0x10,0,0,0,3,(void *)0x1);
          pvVar4 = (void *)(param_1 + 0x54);
          FUN_00435790(pvVar4,(int)pvVar5);
          pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x68),0,0x4000020,0x10,0,0,0,5,(void *)0x1);
          FUN_00435790(pvVar4,(int)pvVar5);
          pvVar5 = FUN_0041a1d0(*(void **)(param_1 + 0x68),0,0x4000020,0x10,0,0,0,(void *)0x1);
          FUN_00435790(pvVar4,(int)pvVar5);
          puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
          local_4._0_1_ = 6;
          FUN_004f26d0(&local_24,puVar6);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar4);
          pvVar5 = FUN_0041a460(*(void **)(param_1 + 0x68),&local_24,0,0x40,2,0,0x20000000,0,
                                (void *)0x1);
          FUN_00435790(pvVar4,(int)pvVar5);
          puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
          local_4._0_1_ = 7;
          FUN_004f26d0(this,puVar6);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar4);
        }
        local_1c = (undefined1 *)0x90;
        local_18 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x68),0,0x4000020,0,0,0,0,3,(void *)0x1);
          pvVar4 = (void *)(param_1 + 0x54);
          FUN_00435790(pvVar4,(int)pvVar5);
          pvVar5 = FUN_0041a1d0(*(void **)(param_1 + 0x68),0,0x20,0,0,0,0,(void *)0x1);
          FUN_00435790(pvVar4,(int)pvVar5);
          puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
          local_4._0_1_ = 8;
          FUN_004f26d0(&local_24,puVar6);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar4);
          pvVar5 = FUN_0041a460(*(void **)(param_1 + 0x68),&local_24,0,0x40,0,0,0x20000000,0,
                                &DAT_00000002);
          FUN_00435790(pvVar4,(int)pvVar5);
          puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_1c);
          local_4._0_1_ = 9;
          FUN_004f26d0(this,puVar6);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar4);
        }
        local_1c = (undefined1 *)0x90;
        local_18 = 0x98;
        if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          FUN_004f26d0((undefined4 *)(param_1 + 0x30),this);
          local_1c = &stack0xffffffc8;
          *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
          iVar1 = *(int *)(param_1 + 0x68);
          FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(param_1 + 0x30));
          pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
          if (pvVar4 != (void *)0x0) {
            *(uint *)((int)pvVar4 + 0x2c) = *(uint *)((int)pvVar4 + 0x2c) | 0x20000000;
          }
        }
        goto LAB_00481e79;
      }
    }
  }
  local_20 = 1;
LAB_00481e87:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_20;
}

