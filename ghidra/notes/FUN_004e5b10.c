
undefined4 __thiscall FUN_004e5b10(void *this,int param_1,uint *param_2)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  void *pvVar5;
  undefined4 *puVar6;
  void *pvVar7;
  uint uVar8;
  uint local_7c;
  uint local_78;
  uint local_74;
  uint local_70;
  undefined4 local_6c;
  uint local_68;
  undefined1 local_64 [4];
  undefined1 local_60 [4];
  undefined1 local_5c [4];
  undefined1 local_58 [4];
  uint local_54;
  undefined1 *local_50;
  undefined4 local_4c;
  undefined4 local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e3d0;
  local_c = ExceptionList;
  local_6c = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_70);
  local_4 = 0;
  FUN_004ece30(&local_7c);
  local_4._0_1_ = 1;
  uVar8 = 0x80;
  local_74 = 0;
  local_78 = 0;
  FUN_004355d0(local_1c);
  local_4._0_1_ = 2;
  local_68 = *(uint *)(*(int *)((int)this + 0xc) + 8);
  if (local_68 != 0) {
    while( true ) {
      local_4c = 0x80;
      local_48 = 0x90;
      if (((*param_2 >> 0x18 < 0x80) || (0x8f < *param_2 >> 0x18)) && (uVar8 != 0)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (!bVar1) break;
      uVar3 = uVar8 & local_68;
      local_54 = uVar8 >> 1;
      uVar8 = local_54;
      if (uVar3 != 0) {
        switch(uVar3) {
        case 1:
        case 0x40:
        case 0x80:
          local_74 = 0x2d;
          local_78 = 6;
          local_6c = 3;
          break;
        case 2:
        case 8:
        case 0x10:
          local_74 = 0x2b;
          local_78 = 4;
          local_6c = 1;
          break;
        case 4:
        case 0x20:
          local_74 = 0x2c;
          local_78 = 5;
          local_6c = 2;
        }
        iVar4 = thunk_FUN_005f5060(*(int *)((int)this + 0x2c));
        if (iVar4 != 0) {
          FUN_004f26d0(&local_70,(undefined4 *)(iVar4 + 0x1c));
        }
        pvVar5 = FUN_00419a00(*(void **)((int)this + 0xc),0,0,2,0,0,0,0xc,&DAT_00000002);
        pvVar7 = (void *)((int)this + 0x1c);
        FUN_00435790(pvVar7,(int)pvVar5);
        local_44 = 0x80;
        local_40 = 0x90;
        if ((local_70 >> 0x18 < 0x80) || (0x8f < local_70 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          puVar6 = FUN_00419b80(*(void **)((int)this + 0xc),&local_70,0,0,2,0,0,0,2);
          FUN_00435790(pvVar7,(int)puVar6);
        }
        if (param_1 != 0) {
          pvVar5 = FUN_00419e90(*(void **)((int)this + 0xc),0,0,2,0,0,0,&DAT_00000002);
          FUN_00435790(pvVar7,(int)pvVar5);
          pvVar5 = FUN_00419e90(*(void **)((int)this + 0xc),0,0,2,0,0,0,&DAT_00000002);
          FUN_00435790(pvVar7,(int)pvVar5);
        }
        puVar6 = FUN_004357b0(pvVar7,local_64);
        local_4._0_1_ = 3;
        FUN_004f26d0(param_2,puVar6);
        local_4._0_1_ = 2;
        FUN_00619730();
        FUN_004ece80(&local_7c);
LAB_004e5d54:
        local_34 = 0x80;
        local_30 = 0x90;
        local_4 = CONCAT31(local_4._1_3_,4);
        if ((*param_2 >> 0x18 < 0x80) || (0x8f < *param_2 >> 0x18)) {
LAB_004e5dbc:
          bVar1 = false;
        }
        else {
          local_3c = 0x90;
          local_38 = 0x98;
          if ((local_7c >> 0x18 < 0x90) || (0x97 < local_7c >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) goto LAB_004e5dbc;
          bVar1 = true;
        }
        local_4._0_1_ = 2;
        FUN_00619730();
        if (bVar1) {
          pvVar7 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x44),param_2);
          if ((pvVar7 != (void *)0x0) &&
             (0 < *(int *)((int)pvVar7 + (local_74 & 0xffff) * 4 + 0x70))) {
            pvVar7 = FUN_0041a340(*(void **)((int)this + 0xc),param_2,0,0,0x20,0,0,0x101,local_78,
                                  &DAT_00000002);
            FUN_00435790(local_1c,(int)pvVar7);
            pvVar7 = FUN_0041a340(*(void **)((int)this + 0xc),param_2,0,0,0x20,0,0,0x101,0x14,
                                  &DAT_00000002);
            FUN_00435790(local_1c,(int)pvVar7);
            puVar6 = FUN_004357b0(local_1c,local_60);
            local_4._0_1_ = 5;
            FUN_004f26d0(&local_7c,puVar6);
            local_4._0_1_ = 2;
            FUN_00619730();
            bVar1 = false;
            while( true ) {
              local_2c = 0x90;
              local_28 = 0x98;
              if (((local_7c >> 0x18 < 0x90) || (0x97 < local_7c >> 0x18)) || (bVar1)) {
                bVar2 = false;
              }
              else {
                bVar2 = true;
              }
              FUN_00619730();
              if (!bVar2) break;
              iVar4 = *(int *)((int)this + 0xc);
              local_50 = &stack0xffffff70;
              FUN_004f26d0(&stack0xffffff70,&local_7c);
              pvVar7 = FUN_00403d30((void *)(iVar4 + 0x2c));
              if ((pvVar7 == (void *)0x0) ||
                 (*(int *)((int)pvVar7 + (local_78 & 0xffff) * 4 + 0x48) < 1)) {
                puVar6 = FUN_004357b0(local_1c,local_5c);
                local_4._0_1_ = 6;
                FUN_004f26d0(&local_7c,puVar6);
                local_4._0_1_ = 2;
                FUN_00619730();
              }
              else {
                bVar1 = true;
              }
            }
            FUN_005f58b0(local_1c);
          }
          local_24 = 0x90;
          local_20 = 0x98;
          if ((local_7c >> 0x18 < 0x90) || (0x97 < local_7c >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) {
            puVar6 = FUN_004357b0((void *)((int)this + 0x1c),local_58);
            local_4._0_1_ = 7;
            FUN_004f26d0(param_2,puVar6);
            local_4._0_1_ = 2;
            FUN_00619730();
          }
          goto LAB_004e5d54;
        }
        FUN_005f58b0((void *)((int)this + 0x1c));
        uVar8 = local_54;
      }
    }
  }
  local_4._0_1_ = 1;
  FUN_00435610(local_1c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_6c;
}

