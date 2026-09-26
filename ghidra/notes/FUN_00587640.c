
int __thiscall
FUN_00587640(void *this,uint *param_1,int param_2,int param_3,int param_4,int param_5,
            undefined4 param_6)

{
  uint uVar1;
  void *pvVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  int iVar6;
  undefined4 uVar7;
  int *piVar8;
  int local_98;
  void *local_94;
  uint local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 auStack_6c [7];
  int iStack_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f889;
  pvStack_c = ExceptionList;
  iVar6 = 1;
  local_98 = 0;
  ExceptionList = &pvStack_c;
  local_94 = this;
  if (*(int *)((int)this + 0x18) != 0) {
    ExceptionList = &pvStack_c;
    if ((*(int *)((int)this + 0x24) != 0) && (ExceptionList = &pvStack_c, param_3 != 0)) {
      ExceptionList = &pvStack_c;
      FUN_005039d0(local_8c,*(undefined4 *)((int)this + 8),1);
      local_4 = 0;
      FUN_00513120((int)local_8c);
      while ((local_70 != 0 && (local_98 == 0))) {
        if (param_5 == 0) {
LAB_005876f7:
          uVar1 = *param_1;
          piVar8 = &local_98;
          uVar7 = 0;
          uVar5 = param_6;
          uVar4 = FUN_0052bed0((int)local_8c);
          iVar3 = (**(code **)(uVar1 + 4))(uVar4,uVar7,piVar8,uVar5);
          if ((iVar3 == 0) || (iVar6 == 0)) {
            iVar6 = 0;
          }
          else {
            iVar6 = 1;
          }
        }
        else {
          iVar3 = FUN_0052bed0((int)local_8c);
          local_90 = *(uint *)(iVar3 + 0x58) & 1;
          if (local_90 == 0) goto LAB_005876f7;
        }
        FUN_005130d0((int)local_8c);
      }
      local_4 = 0xffffffff;
      FUN_00503ad0(local_8c);
    }
    if ((*(int *)((int)local_94 + 0x28) != 0) && (param_4 != 0)) {
      FUN_00504c40(local_8c,*(undefined4 *)((int)local_94 + 8),1);
      local_4 = 1;
      FUN_00513120((int)local_8c);
      while ((local_70 != 0 && (local_98 == 0))) {
        if (param_5 == 0) {
LAB_005877b8:
          uVar1 = *param_1;
          piVar8 = &local_98;
          uVar7 = 0;
          uVar5 = param_6;
          uVar4 = FUN_0052bed0((int)local_8c);
          iVar3 = (**(code **)(uVar1 + 4))(uVar4,uVar7,piVar8,uVar5);
          if ((iVar3 == 0) || (iVar6 == 0)) {
            iVar6 = 0;
          }
          else {
            iVar6 = 1;
          }
        }
        else {
          iVar3 = FUN_0052bed0((int)local_8c);
          local_90 = *(uint *)(iVar3 + 0x58) & 1;
          if (local_90 == 0) goto LAB_005877b8;
        }
        FUN_005130d0((int)local_8c);
      }
      local_4 = 0xffffffff;
      FUN_00504d40(local_8c);
    }
  }
  pvVar2 = local_94;
  if (*(int *)((int)local_94 + 0x1c) != 0) {
    FUN_004ffe70(local_8c,*(undefined4 *)((int)local_94 + 8),1);
    local_4 = 2;
    FUN_00513120((int)local_8c);
    while ((local_70 != 0 && (local_98 == 0))) {
      uVar4 = 1;
      uVar5 = FUN_0052bed0((int)local_8c);
      FUN_00502db0(auStack_6c,uVar5,uVar4);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00513120((int)auStack_6c);
      while ((iStack_50 != 0 && (local_98 == 0))) {
        if ((*(int *)((int)pvVar2 + 0x20) != 0) && (param_2 != 0)) {
          if (param_5 != 0) {
            iVar3 = FUN_0052bed0((int)auStack_6c);
            local_90 = *(uint *)(iVar3 + 0x58) & 1;
            if (local_90 != 0) goto LAB_005878ff;
          }
          uVar4 = FUN_0052bed0((int)auStack_6c);
          local_90 = *param_1;
          piVar8 = &local_98;
          uVar5 = param_6;
          uVar7 = FUN_0052bed0((int)local_8c);
          iVar3 = (**(code **)(local_90 + 4))(uVar4,uVar7,piVar8,uVar5);
          if ((iVar3 == 0) || (iVar6 == 0)) {
            iVar6 = 0;
          }
          else {
            iVar6 = 1;
          }
        }
LAB_005878ff:
        if ((*(int *)((int)local_94 + 0x24) != 0) && (param_3 != 0)) {
          uVar4 = 1;
          uVar5 = FUN_0052bed0((int)auStack_6c);
          FUN_005039d0(auStack_4c,uVar5,uVar4);
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00513120((int)auStack_4c);
          while ((iStack_30 != 0 && (local_98 == 0))) {
            if (param_5 == 0) {
LAB_00587973:
              uVar4 = FUN_0052bed0((int)auStack_4c);
              uVar1 = *param_1;
              piVar8 = &local_98;
              uVar5 = param_6;
              uVar7 = FUN_0052bed0((int)local_8c);
              iVar3 = (**(code **)(uVar1 + 4))(uVar4,uVar7,piVar8,uVar5);
              if ((iVar3 == 0) || (iVar6 == 0)) {
                iVar6 = 0;
              }
              else {
                iVar6 = 1;
              }
            }
            else {
              iVar3 = FUN_0052bed0((int)auStack_4c);
              local_90 = *(uint *)(iVar3 + 0x58) & 1;
              if (local_90 == 0) goto LAB_00587973;
            }
            FUN_005130d0((int)auStack_4c);
          }
          local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00503ad0(auStack_4c);
        }
        if ((*(int *)((int)local_94 + 0x28) != 0) && (param_4 != 0)) {
          uVar4 = 1;
          uVar5 = FUN_0052bed0((int)auStack_6c);
          FUN_00504c40(auStack_2c,uVar5,uVar4);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00513120((int)auStack_2c);
          while ((iStack_10 != 0 && (local_98 == 0))) {
            if (param_5 == 0) {
LAB_00587a4b:
              uVar4 = FUN_0052bed0((int)auStack_2c);
              uVar1 = *param_1;
              piVar8 = &local_98;
              uVar5 = param_6;
              uVar7 = FUN_0052bed0((int)local_8c);
              iVar3 = (**(code **)(uVar1 + 4))(uVar4,uVar7,piVar8,uVar5);
              if ((iVar3 == 0) || (iVar6 == 0)) {
                iVar6 = 0;
              }
              else {
                iVar6 = 1;
              }
            }
            else {
              iVar3 = FUN_0052bed0((int)auStack_2c);
              local_90 = *(uint *)(iVar3 + 0x58) & 1;
              if (local_90 == 0) goto LAB_00587a4b;
            }
            FUN_005130d0((int)auStack_2c);
          }
          local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00504d40(auStack_2c);
        }
        FUN_005130d0((int)auStack_6c);
        pvVar2 = local_94;
      }
      local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00502eb0(auStack_6c);
      FUN_005130d0((int)local_8c);
    }
    local_4 = 0xffffffff;
    FUN_004fff70(local_8c);
  }
  ExceptionList = pvStack_c;
  return iVar6;
}

