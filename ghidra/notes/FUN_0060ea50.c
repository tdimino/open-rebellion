
uint __thiscall
FUN_0060ea50(void *this,uint *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            int param_7)

{
  undefined1 *puVar1;
  byte bVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  byte *pbVar12;
  undefined1 *puVar13;
  undefined4 *puVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  undefined4 *puVar17;
  int local_20;
  int local_1c;
  
  iVar3 = FUN_005fc110(this);
  if (iVar3 == 1) {
    uVar4 = FUN_0060ed00(this,param_1);
    return uVar4;
  }
  iVar3 = FUN_00583c40((int)param_1);
  iVar5 = FUN_005fc0e0((int *)param_1);
  iVar6 = FUN_005fc0f0((int *)param_1);
  uVar4 = FUN_005fc9c0(param_1);
  iVar7 = FUN_00583c40((int)this);
  iVar8 = FUN_005fc0e0(this);
  uVar9 = FUN_005fc9c0(this);
  uVar10 = FUN_005fc100(this);
  if (iVar6 < 0) {
    local_20 = 1;
    iVar6 = -iVar6;
  }
  else {
    local_20 = -1;
  }
  iVar11 = FUN_005fc0f0(this);
  if (iVar11 < 0) {
    iVar11 = -iVar11;
    local_1c = 1;
  }
  else {
    local_1c = -1;
  }
  if (param_5 <= iVar11) {
    if (param_6 == 0) {
      if (param_2 < 0) {
        param_2 = 0;
      }
      param_6 = iVar5 - param_2;
    }
    if (param_7 == 0) {
      if (param_3 < 0) {
        param_3 = 0;
      }
      param_7 = iVar6 - param_3;
    }
    if (param_2 < 0) {
      param_6 = param_6 + param_2;
      param_2 = 0;
    }
    if (param_3 < 0) {
      param_7 = param_7 + param_3;
      param_3 = 0;
    }
    if (iVar11 - param_5 < param_7) {
      param_7 = iVar11 - param_5;
    }
    if (iVar8 - param_4 < param_6) {
      param_6 = iVar8 - param_4;
    }
    if (param_4 < 0) {
      param_6 = param_6 + param_4;
      if (param_6 < 1) {
        return 0;
      }
      param_4 = 0;
    }
    if (param_5 < 0) {
      param_7 = param_7 + param_5;
      if (param_7 < 1) {
        return 0;
      }
      param_5 = 0;
    }
    iVar5 = FUN_005fc9e0(param_1,param_3);
    iVar6 = FUN_005fc9e0(this,param_5);
    if (uVar10 == 0x18) {
      if (0 < param_7) {
        puVar15 = (undefined4 *)(param_2 * 3 + iVar3 + iVar5 * uVar4);
        iVar3 = param_4 * 3;
        param_4 = param_7;
        puVar14 = (undefined4 *)(iVar3 + iVar7 + iVar6 * uVar9);
        do {
          puVar16 = puVar14;
          puVar17 = puVar15;
          for (uVar10 = (uint)(param_6 * 3) >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
            *puVar17 = *puVar16;
            puVar16 = puVar16 + 1;
            puVar17 = puVar17 + 1;
          }
          puVar15 = (undefined4 *)((int)puVar15 + uVar4 * local_20);
          for (uVar10 = param_6 * 3 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
            *(undefined1 *)puVar17 = *(undefined1 *)puVar16;
            puVar16 = (undefined4 *)((int)puVar16 + 1);
            puVar17 = (undefined4 *)((int)puVar17 + 1);
          }
          puVar14 = (undefined4 *)((int)puVar14 + uVar9 * local_1c);
          param_4 = param_4 + -1;
        } while (param_4 != 0);
      }
    }
    else if (0 < param_7) {
      pbVar12 = (byte *)(iVar6 * uVar9 + iVar7 + param_4);
      puVar13 = (undefined1 *)(param_2 * 3 + 2 + iVar3 + iVar5 * uVar4);
      iVar3 = param_7;
      do {
        bVar2 = *pbVar12;
        pbVar12 = pbVar12 + uVar9 * local_1c;
        puVar1 = (undefined1 *)(*(int *)this + 0x28 + (uint)bVar2 * 4);
        puVar13[-2] = *puVar1;
        puVar13[-1] = puVar1[1];
        *puVar13 = puVar1[2];
        puVar13 = puVar13 + uVar4 * local_20;
        iVar3 = iVar3 + -1;
      } while (iVar3 != 0);
    }
    return (uint)(0 < param_7);
  }
  return 0;
}

