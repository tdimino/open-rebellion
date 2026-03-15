
bool __thiscall FUN_0050d5a0(void *this,undefined4 param_1,int param_2,int param_3)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  uint *puVar4;
  int iVar5;
  void *pvVar6;
  undefined4 uVar7;
  uint *puVar8;
  bool bVar9;
  uint *puVar10;
  int iVar11;
  undefined4 uVar12;
  int *local_174;
  uint local_16c [5];
  void *local_158;
  int local_154;
  undefined4 local_150;
  uint local_14c [6];
  undefined4 local_134;
  undefined4 local_130;
  uint local_11c [2];
  undefined1 local_114 [8];
  undefined1 local_10c [8];
  undefined1 local_104 [8];
  undefined4 local_fc;
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0;
  undefined4 local_ec;
  undefined4 local_e8;
  undefined4 local_e4;
  undefined4 local_e0;
  undefined4 local_dc;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined4 local_d0;
  undefined4 local_cc;
  undefined4 local_c8;
  undefined4 local_c4;
  undefined4 local_c0;
  undefined4 local_bc;
  undefined4 local_b8;
  undefined4 local_b4;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined4 local_98;
  undefined4 local_94;
  undefined4 local_90;
  undefined4 local_8c;
  undefined4 local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined4 local_74;
  undefined4 local_70;
  undefined4 local_6c;
  undefined4 local_68;
  undefined4 local_64;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641f83;
  local_c = ExceptionList;
  bVar9 = true;
  local_174 = (int *)0x0;
  local_158 = this;
  if (param_2 == 1) {
    iVar11 = 1;
  }
  else {
    ExceptionList = &local_c;
    if (param_2 != 2) goto LAB_0050d5ed;
    iVar11 = 2;
  }
  ExceptionList = &local_c;
  local_174 = (int *)FUN_00539fd0(this,iVar11);
LAB_0050d5ed:
  if (local_174 != (int *)0x0) {
    bVar2 = false;
    bVar1 = false;
    local_16c[4] = 0;
    local_154 = 0;
    local_14c[2] = 0;
    local_150 = 0;
    FUN_00618c80(local_11c,8,4,FUN_00520570);
    local_4 = 0;
    FUN_00618c80(local_16c,8,1,FUN_00520570);
    local_4._0_1_ = 1;
    FUN_00618c80(local_14c,8,1,FUN_00520570);
    local_4._0_1_ = 2;
    bVar3 = false;
    FUN_004fd450(local_14c + 5,param_3);
    local_4 = CONCAT31(local_4._1_3_,3);
    local_134 = 1;
    local_130 = 3;
    switch(param_1) {
    case 0:
      break;
    case 1:
      bVar2 = true;
      local_16c[4] = 1;
      break;
    case 2:
      bVar2 = true;
      local_14c[2] = 1;
      break;
    case 3:
      bVar2 = true;
      local_84 = 0x10;
      local_80 = 0x20;
      local_4._0_1_ = 6;
      FUN_00520580(local_11c,&local_84);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_74 = 0x22;
      local_70 = 0x28;
      local_4._0_1_ = 7;
      FUN_00520580(local_114,&local_74);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 4:
      bVar2 = true;
      local_cc = 0xa0;
      local_c8 = 0xb0;
      local_4._0_1_ = 4;
      FUN_00520580(local_11c,&local_cc);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_94 = 0x28;
      local_90 = 0x30;
      local_4._0_1_ = 5;
      FUN_00520580(local_114,&local_94);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 5:
      bVar2 = true;
      local_64 = 8;
      local_60 = 0x10;
      local_4._0_1_ = 8;
      FUN_00520580(local_11c,&local_64);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_ec = 1;
      local_e8 = 0xff;
      local_4._0_1_ = 9;
      FUN_00520580(local_16c,&local_ec);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 6:
      bVar2 = true;
      local_6c = 8;
      local_16c[4] = 1;
      local_154 = 1;
      local_68 = 0x10;
      local_4._0_1_ = 10;
      FUN_00520580(local_11c,&local_6c);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_d8 = 0xff;
      local_dc = 1;
      local_4._0_1_ = 0xb;
      FUN_00520580(local_16c,&local_dc);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      bVar3 = true;
      break;
    case 7:
      bVar2 = true;
      local_8c = 8;
      local_88 = 0x10;
      local_4._0_1_ = 0xc;
      FUN_00520580(local_11c,&local_8c);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_fc = 1;
      local_f8 = 0xff;
      local_4._0_1_ = 0xd;
      FUN_00520580(local_16c,&local_fc);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_54 = 1;
      local_50 = 0xff;
      local_4._0_1_ = 0xe;
      FUN_00520580(local_14c,&local_54);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 8:
      local_bc = 0x20;
      local_150 = 1;
      local_b8 = 0x30;
      local_4._0_1_ = 0xf;
      FUN_00520580(local_11c,&local_bc);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_78 = 0x20;
      local_7c = 0x10;
      local_4._0_1_ = 0x10;
      FUN_00520580(local_114,&local_7c);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_ac = 0xa0;
      local_a8 = 0xb0;
      local_4._0_1_ = 0x11;
      FUN_00520580(local_10c,&local_ac);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_5c = 8;
      local_58 = 0x10;
      local_4._0_1_ = 0x12;
      FUN_00520580(local_104,&local_5c);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_9c = 1;
      local_98 = 0xff;
      local_4._0_1_ = 0x13;
      FUN_00520580(local_16c,&local_9c);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_f4 = 0x10;
      local_f0 = 0x20;
      local_4._0_1_ = 0x14;
      FUN_00520580(local_14c,&local_f4);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      bVar1 = true;
      break;
    case 9:
      bVar2 = true;
      local_e4 = 0x30;
      local_e0 = 0x40;
      local_4._0_1_ = 0x15;
      FUN_00520580(local_11c,&local_e4);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 10:
      bVar1 = true;
      local_d4 = 1;
      local_d0 = 0xff;
      local_4._0_1_ = 0x16;
      FUN_00520580(local_11c,&local_d4);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_c4 = 1;
      local_c0 = 0xff;
      local_4._0_1_ = 0x17;
      FUN_00520580(local_16c,&local_c4);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_b4 = 1;
      local_b0 = 0xff;
      local_4._0_1_ = 0x18;
      FUN_00520580(local_14c,&local_b4);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 0xb:
      bVar1 = true;
      local_a4 = 0x10;
      local_a0 = 0x40;
      local_4._0_1_ = 0x19;
      FUN_00520580(local_11c,&local_a4);
      local_4._0_1_ = 3;
      FUN_00619730();
      local_14c[3] = 0xa0;
      local_14c[4] = 0xb0;
      local_4._0_1_ = 0x1a;
      FUN_00520580(local_114,local_14c + 3);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    case 0xc:
      bVar2 = true;
      local_16c[2] = 0x20;
      local_16c[3] = 0x30;
      local_4._0_1_ = 0x1b;
      FUN_00520580(local_11c,local_16c + 2);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      break;
    default:
      bVar9 = false;
    }
    if (bVar9) {
      if (bVar1) {
        iVar11 = (**(code **)(*local_174 + 0x24))(local_14c + 5);
        bVar9 = iVar11 != 0;
      }
      else {
        if (bVar2) {
          iVar11 = FUN_0050fc80(local_174,local_14c + 5);
          iVar5 = FUN_0050fbf0(local_174,local_14c + 5);
          if ((iVar5 == 0) || (iVar11 == 0)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
          iVar11 = FUN_004f9ca0(local_174,local_14c + 5);
          if ((iVar11 == 0) || (!bVar9)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
        }
        if (bVar9) {
          if (local_16c[4] != 0) {
            iVar11 = FUN_005103a0(local_174,local_14c + 5);
            iVar5 = FUN_0050fa90(local_174,local_14c + 5);
            if ((iVar5 == 0) || (iVar11 == 0)) {
              bVar9 = false;
            }
            else {
              bVar9 = true;
            }
            iVar11 = FUN_0050f390(local_174,local_14c + 5);
            if ((iVar11 == 0) || (!bVar9)) {
              bVar9 = false;
            }
            else {
              bVar9 = true;
            }
          }
          if (bVar9) {
            if (local_154 != 0) {
              iVar11 = FUN_0050fd10(local_174,local_14c + 5);
              iVar5 = FUN_0050f840(local_174,local_14c + 5);
              if ((iVar5 == 0) || (iVar11 == 0)) {
                bVar9 = false;
              }
              else {
                bVar9 = true;
              }
            }
            if ((bVar9) && (local_14c[2] != 0)) {
              iVar11 = FUN_0050f410(local_174,local_14c + 5);
              iVar5 = FUN_0050f490(local_174,local_14c + 5);
              if ((iVar5 == 0) || (iVar11 == 0)) {
                bVar9 = false;
              }
              else {
                bVar9 = true;
              }
            }
          }
        }
      }
    }
    iVar11 = 0;
    puVar8 = local_11c;
    do {
      puVar4 = (uint *)FUN_00520570(local_16c + 2);
      if ((*puVar4 == *puVar8) && (puVar4[1] == puVar8[1])) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) break;
      iVar5 = FUN_004f7e30(local_174,puVar8,local_14c + 5);
      if ((iVar5 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
      iVar11 = iVar11 + 1;
      puVar8 = puVar8 + 2;
    } while (iVar11 < 4);
    if (bVar3) {
      iVar11 = FUN_0050eea0(local_174,local_14c + 5);
      if ((iVar11 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    if (bVar9) {
      FUN_004ffe70(auStack_2c,local_174,4);
      local_4 = CONCAT31(local_4._1_3_,0x1c);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        iVar11 = 0;
        if (bVar9) {
          puVar8 = local_16c;
          do {
            if (0 < iVar11) break;
            puVar4 = (uint *)FUN_00520570(local_16c + 2);
            if ((*puVar4 == *puVar8) && (puVar4[1] == puVar8[1])) {
              bVar1 = true;
            }
            else {
              bVar1 = false;
            }
            FUN_00619730();
            if (bVar1) break;
            puVar4 = local_14c + 5;
            puVar10 = puVar8;
            pvVar6 = (void *)FUN_0052bed0((int)auStack_2c);
            iVar5 = FUN_004f7e30(pvVar6,puVar10,puVar4);
            if ((iVar5 == 0) || (!bVar9)) {
              bVar9 = false;
            }
            else {
              bVar9 = true;
            }
            iVar11 = iVar11 + 1;
            puVar8 = puVar8 + 2;
          } while (bVar9);
        }
        uVar12 = 4;
        uVar7 = FUN_0052bed0((int)auStack_2c);
        FUN_00502db0(auStack_4c,uVar7,uVar12);
        local_4 = CONCAT31(local_4._1_3_,0x1d);
        FUN_00513120((int)auStack_4c);
        while (iStack_30 != 0) {
          iVar11 = 0;
          puVar8 = local_14c;
          do {
            puVar4 = (uint *)FUN_00520570(local_14c + 3);
            if ((*puVar4 == *puVar8) && (puVar4[1] == puVar8[1])) {
              bVar1 = true;
            }
            else {
              bVar1 = false;
            }
            FUN_00619730();
            if (bVar1) break;
            puVar4 = local_14c + 5;
            puVar10 = puVar8;
            pvVar6 = (void *)FUN_0052bed0((int)auStack_4c);
            iVar5 = FUN_004f7e30(pvVar6,puVar10,puVar4);
            if ((iVar5 == 0) || (!bVar9)) {
              bVar9 = false;
            }
            else {
              bVar9 = true;
            }
            iVar11 = iVar11 + 1;
            puVar8 = puVar8 + 2;
          } while (iVar11 < 1);
          if (bVar3) {
            puVar8 = local_14c + 5;
            pvVar6 = (void *)FUN_0052bed0((int)auStack_4c);
            iVar11 = FUN_0050eea0(pvVar6,puVar8);
            if ((iVar11 == 0) || (!bVar9)) {
              bVar9 = false;
            }
            else {
              bVar9 = true;
            }
          }
          FUN_005130d0((int)auStack_4c);
        }
        local_4._1_3_ = (uint3)((uint)local_4 >> 8);
        local_4 = CONCAT31(local_4._1_3_,0x1c);
        FUN_00502eb0(auStack_4c);
        FUN_005130d0((int)auStack_2c);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_004fff70(auStack_2c);
    }
    local_4._0_1_ = 2;
    FUN_004fd4d0(local_14c + 5);
    local_4._0_1_ = 1;
    FUN_00618d20(local_14c,8,1,FUN_00619730);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00618d20(local_16c,8,1,FUN_00619730);
    local_4 = 0xffffffff;
    FUN_00618d20(local_11c,8,4,FUN_00619730);
  }
  ExceptionList = local_c;
  return bVar9;
}

