
undefined4 FUN_0046d8d0(int *param_1,int param_2,HDC param_3)

{
  byte *pbVar1;
  ushort uVar2;
  ushort uVar3;
  ushort *puVar4;
  undefined4 uVar5;
  void *pvVar6;
  undefined4 *puVar7;
  int iVar8;
  char *pcVar9;
  HGDIOBJ h;
  int iVar10;
  int *piVar11;
  uint uVar12;
  undefined4 uVar13;
  ushort *local_d4;
  int local_d0;
  int iStack_cc;
  int local_c8;
  int iStack_c4;
  ushort local_bc;
  ushort local_ba;
  int local_b8 [2];
  undefined1 local_b0 [8];
  undefined1 local_a8 [8];
  undefined1 local_a0 [8];
  undefined1 local_98 [4];
  int local_94;
  undefined1 auStack_90 [4];
  undefined4 auStack_8c [7];
  int iStack_70;
  undefined4 local_6c [3];
  undefined4 auStack_60 [7];
  int iStack_44;
  undefined4 local_40 [11];
  undefined4 uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006321de;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00618c80(&local_bc,8,5,FUN_00465b90);
  local_4 = 0;
  FUN_00601880(local_40,(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 1;
  FUN_005f2f50(local_6c);
  local_4._0_1_ = 2;
  FUN_004ece30(&local_c8);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_0042dbe0(&local_d0);
  puVar4 = &local_ba;
  iVar10 = 5;
  do {
    puVar4[-1] = 0;
    *puVar4 = 0;
    puVar4 = puVar4 + 4;
    iVar10 = iVar10 + -1;
  } while (iVar10 != 0);
  if (param_2 == 1) {
    local_d4 = (ushort *)0x10000002;
    local_4._0_1_ = 5;
    FUN_004f26d0(local_b8,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000001;
    local_4._0_1_ = 6;
    FUN_004f26d0(local_b0,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000005;
    local_4._0_1_ = 7;
    FUN_004f26d0(local_a8,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000003;
    local_4._0_1_ = 8;
    FUN_004f26d0(local_a0,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000004;
    local_4._0_1_ = 9;
    FUN_004f26d0(local_98,&local_d4);
  }
  else {
    local_d4 = (ushort *)0x10000008;
    local_4._0_1_ = 10;
    FUN_004f26d0(local_b8,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x1000000a;
    local_4._0_1_ = 0xb;
    FUN_004f26d0(local_b0,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000007;
    local_4._0_1_ = 0xc;
    FUN_004f26d0(local_a8,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000006;
    local_4._0_1_ = 0xd;
    FUN_004f26d0(local_a0,&local_d4);
    local_4._0_1_ = 4;
    FUN_00619730();
    local_d4 = (ushort *)0x10000009;
    local_4._0_1_ = 0xe;
    FUN_004f26d0(local_98,&local_d4);
  }
  local_4._0_1_ = 4;
  FUN_00619730();
  iVar10 = (**(code **)(*param_1 + 4))();
  if (iVar10 == 8) {
    FUN_00502db0(auStack_8c,param_1,3);
    local_4 = CONCAT31(local_4._1_3_,0xf);
    FUN_00513120((int)auStack_8c);
    while (iStack_70 != 0) {
      uVar13 = 3;
      iVar10 = param_2;
      uVar5 = FUN_0052bed0((int)auStack_8c);
      FUN_00504cc0(auStack_60,uVar5,uVar13,iVar10);
      local_4 = CONCAT31(local_4._1_3_,0x10);
      FUN_00513120((int)auStack_60);
      while (iStack_44 != 0) {
        pvVar6 = (void *)FUN_0052bed0((int)auStack_60);
        puVar7 = FUN_00402d80(pvVar6,&iStack_cc);
        local_4._0_1_ = 0x11;
        FUN_004f26d0(&local_d0,puVar7);
        local_4 = CONCAT31(local_4._1_3_,0x10);
        FUN_00619730();
        iVar10 = 0;
        piVar11 = local_b8;
        do {
          if (local_d0 == *piVar11) {
            iVar8 = iVar10 * 8;
            (&local_bc)[iVar10 * 4] = (&local_bc)[iVar10 * 4] + 1;
            pbVar1 = (byte *)((int)local_b8 + iVar8 + -2);
            *pbVar1 = *pbVar1 | 1;
            if ((*(byte *)((int)pvVar6 + 0x50) & 1) == 0) {
              *(ushort *)((int)local_b8 + iVar8 + -2) = *(ushort *)((int)local_b8 + iVar8 + -2) | 4;
            }
            break;
          }
          iVar10 = iVar10 + 1;
          piVar11 = piVar11 + 2;
        } while (iVar10 < 5);
        FUN_005130d0((int)auStack_60);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,0xf);
      FUN_00504d40(auStack_60);
      FUN_005130d0((int)auStack_8c);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00502eb0(auStack_8c);
  }
  else {
    FUN_00504cc0(auStack_8c,param_1,3,param_2);
    local_4 = CONCAT31(local_4._1_3_,0x12);
    FUN_00513120((int)auStack_8c);
    while (iStack_70 != 0) {
      pvVar6 = (void *)FUN_0052bed0((int)auStack_8c);
      puVar7 = FUN_00402d80(pvVar6,auStack_90);
      local_4._0_1_ = 0x13;
      FUN_004f26d0(&local_d0,puVar7);
      local_4 = CONCAT31(local_4._1_3_,0x12);
      FUN_00619730();
      iVar10 = 0;
      piVar11 = local_b8;
      do {
        if (local_d0 == *piVar11) {
          iVar8 = iVar10 * 8;
          (&local_bc)[iVar10 * 4] = (&local_bc)[iVar10 * 4] + 1;
          pbVar1 = (byte *)((int)local_b8 + iVar8 + -2);
          *pbVar1 = *pbVar1 | 1;
          if ((*(byte *)((int)pvVar6 + 0x50) & 1) == 0) {
            *(ushort *)((int)local_b8 + iVar8 + -2) = *(ushort *)((int)local_b8 + iVar8 + -2) | 4;
          }
          break;
        }
        iVar10 = iVar10 + 1;
        piVar11 = piVar11 + 2;
      } while (iVar10 < 5);
      FUN_005130d0((int)auStack_8c);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00504d40(auStack_8c);
  }
  FUN_00601c60(local_40,10);
  FUN_00403e90(local_40,1);
  uVar12 = 5;
  puVar4 = &local_ba;
  uStack_14 = 1;
  local_40[0] = 0x1c;
  iStack_c4 = 0xc2;
  iStack_cc = 5;
  iVar10 = local_c8;
  do {
    local_d4 = puVar4;
    FUN_005f3c20(local_6c);
    FUN_005f31f0(local_6c,(int)(short)puVar4[-1]);
    FUN_00601b30(local_40,iStack_c4,2);
    pcVar9 = (char *)FUN_00583c40((int)local_6c);
    FUN_00601aa0(local_40,pcVar9);
    if (puVar4[-1] == 0) {
      FUN_00601aa0(local_40,&DAT_006a8788);
      iVar10 = 0xffffff;
      uVar12 = 5;
    }
    else {
      uVar2 = *puVar4;
      uVar3 = uVar2 & 1;
      if (uVar3 == 0) {
LAB_0046de78:
        if ((uVar2 & 4) != 0) {
          iVar10 = 0xffffff;
          uVar12 = 2;
        }
      }
      else if ((uVar2 & 4) == 0) {
        if (param_2 == 2) {
          iVar10 = 0;
          uVar12 = 1;
        }
        else {
          iVar10 = 0xffffff;
          uVar12 = 0;
        }
      }
      else {
        if (uVar3 == 0) goto LAB_0046de78;
        if ((uVar2 & 4) == 0) {
          if (uVar3 == 0) goto LAB_0046de78;
        }
        else if (param_2 == 2) {
          iVar10 = 0;
          uVar12 = 4;
        }
        else {
          iVar10 = 0xffffff;
          uVar12 = 3;
        }
      }
    }
    if (uVar12 < 5) {
      h = SelectObject(param_3,*(HGDIOBJ *)(local_94 + 0x164 + uVar12 * 4));
      PatBlt(param_3,iStack_c4,2,0x15,0x10,0xf00021);
      SelectObject(param_3,h);
      puVar4 = local_d4;
    }
    FUN_00601c90(local_40,iVar10,(HWND)0x0);
    FUN_00601ce0(local_40,param_3);
    iStack_c4 = iStack_c4 + 0x1c;
    puVar4 = puVar4 + 4;
    iStack_cc = iStack_cc + -1;
    if (iStack_cc == 0) {
      local_4._0_1_ = 3;
      local_d4 = puVar4;
      FUN_00619730();
      local_4._0_1_ = 2;
      FUN_00619730();
      local_4._0_1_ = 1;
      FUN_005f2ff0(local_6c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00601990((int)local_40);
      local_4 = 0xffffffff;
      FUN_00618d20(&local_bc,8,5,&LAB_00465ba0);
      ExceptionList = pvStack_c;
      return 0;
    }
  } while( true );
}

