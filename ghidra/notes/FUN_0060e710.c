
void __thiscall
FUN_0060e710(void *this,uint *param_1,int param_2,int param_3,undefined4 param_4,int param_5,
            byte *param_6,int param_7,int param_8)

{
  undefined4 *puVar1;
  uint *this_00;
  int iVar2;
  uint *puVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  byte *pbVar12;
  int iVar13;
  byte *pbVar14;
  undefined1 *puVar15;
  int iVar16;
  byte *pbVar17;
  int iVar18;
  undefined1 *puVar19;
  int local_30;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_00656cc8;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  iVar2 = FUN_00583c40((int)param_1);
  puVar3 = (uint *)FUN_005fc0f0((int *)param_1);
  iVar4 = FUN_005fc0e0((int *)param_1);
  uVar5 = FUN_005fc9c0(param_1);
  iVar6 = FUN_00583c40((int)this);
  local_30 = FUN_005fc0f0(this);
  iVar7 = FUN_005fc0e0(this);
  uVar8 = FUN_005fc9c0(this);
  uVar9 = FUN_005fc100(this);
  if (param_3 < 0) {
    param_6 = param_6 + -param_3;
    if (param_8 != 0) {
      param_8 = param_8 - param_3;
    }
    param_3 = 0;
  }
  if (param_2 < 0) {
    param_5 = param_5 - param_2;
    if (param_7 != 0) {
      param_7 = param_7 - param_2;
    }
    param_2 = 0;
  }
  iVar16 = param_5;
  if ((int)puVar3 < 0) {
    iVar13 = 1;
    param_1 = (uint *)-(int)puVar3;
  }
  else {
    iVar13 = -1;
    param_1 = puVar3;
  }
  if (local_30 < 0) {
    local_30 = -local_30;
    iVar18 = 1;
  }
  else {
    iVar18 = -1;
  }
  iVar10 = FUN_005fc9e0(this_00,param_3);
  iVar11 = FUN_005fc9e0(this,(int)param_6);
  if (param_7 == 0) {
    param_7 = iVar7 - iVar16;
  }
  if (param_8 == 0) {
    param_8 = local_30 - (int)param_6;
  }
  if ((int)param_1 - param_3 < param_8) {
    param_8 = (int)param_1 - param_3;
  }
  if (iVar4 - param_2 < param_7) {
    param_7 = iVar4 - param_2;
  }
  if (uVar9 == 0x18) {
    if (0 < param_8) {
      iVar10 = iVar10 * uVar5;
      param_6 = (byte *)(iVar11 * uVar8);
      pbVar12 = param_6 + iVar6 + iVar16 * 3;
      iVar4 = param_7;
      do {
        if (0 < iVar4) {
          pbVar17 = pbVar12 + iVar10 + (((iVar2 + (param_2 - iVar16) * 3) - iVar6) - (int)param_6) +
                                       2;
          pbVar14 = pbVar12;
          iVar7 = iVar4;
          do {
            iVar4 = FUN_0060e620(&param_4,*(undefined4 *)pbVar14);
            if (iVar4 != 0) {
              pbVar17[-2] = *pbVar14;
              pbVar17[-1] = pbVar14[1];
              *pbVar17 = pbVar14[2];
            }
            pbVar14 = pbVar14 + 3;
            pbVar17 = pbVar17 + 3;
            iVar7 = iVar7 + -1;
            iVar4 = param_7;
            iVar16 = param_5;
          } while (iVar7 != 0);
        }
        param_6 = param_6 + iVar18 * uVar8;
        pbVar12 = pbVar12 + iVar18 * uVar8;
        iVar10 = iVar10 + iVar13 * uVar5;
        param_8 = param_8 + -1;
      } while (param_8 != 0);
    }
  }
  else if (0 < param_8) {
    param_5 = iVar18 * uVar8;
    pbVar12 = (byte *)(iVar11 * uVar8 + iVar6 + iVar16);
    puVar15 = (undefined1 *)(param_2 * 3 + iVar2 + iVar10 * uVar5);
    param_2 = param_8;
    iVar2 = param_7;
    do {
      puVar19 = puVar15;
      param_6 = pbVar12;
      if (0 < iVar2) {
        do {
          param_8 = iVar2;
          puVar1 = (undefined4 *)(*(int *)this + 0x28 + (uint)*pbVar12 * 4);
          iVar2 = FUN_0060e620(&param_4,*puVar1);
          if (iVar2 != 0) {
            *puVar19 = *(undefined1 *)puVar1;
            puVar19[1] = *(undefined1 *)((int)puVar1 + 1);
            puVar19[2] = *(undefined1 *)((int)puVar1 + 2);
          }
          pbVar12 = pbVar12 + 1;
          puVar19 = puVar19 + 3;
          iVar2 = param_8 + -1;
        } while (param_8 + -1 != 0);
        param_8 = 0;
        iVar2 = param_7;
      }
      puVar15 = puVar15 + iVar13 * uVar5;
      pbVar12 = param_6 + param_5;
      param_2 = param_2 + -1;
      param_6 = pbVar12;
    } while (param_2 != 0);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

