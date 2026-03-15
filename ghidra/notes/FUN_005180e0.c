
int __cdecl FUN_005180e0(void *param_1)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  uint uVar15;
  bool bVar16;
  int local_54;
  uint local_44;
  uint local_3c [6];
  int local_24;
  int local_20;
  uint local_1c;
  uint local_18;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642a58;
  local_c = ExceptionList;
  uVar13 = 0;
  local_1c = 0;
  bVar1 = false;
  local_10 = 0;
  local_14 = 0;
  local_18 = 0;
  local_44 = 0;
  bVar16 = false;
  bVar3 = false;
  bVar5 = false;
  bVar6 = false;
  ExceptionList = &local_c;
  iVar7 = thunk_FUN_00506e10();
  local_3c[0] = 0x32000242;
  local_4 = 0;
  iVar8 = FUN_00505190(local_3c);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_3c[1] = 0x30000240;
  local_4 = 1;
  local_3c[5] = FUN_00505190(local_3c + 1);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_3c[2] = 0x35000281;
  local_4 = 2;
  local_24 = FUN_00505190(local_3c + 2);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_3c[3] = 0x34000280;
  local_4 = 3;
  local_20 = FUN_00505190(local_3c + 3);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_3c[4] = 0x90000109;
  local_4 = 4;
  iVar9 = FUN_00505f60(local_3c + 4);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar10 = FUN_00506110();
  bVar4 = false;
  if (((((iVar8 == 0) || (local_3c[5] == 0)) || (local_24 == 0)) ||
      ((local_20 == 0 || (iVar9 == 0)))) || (local_54 = 1, iVar7 == 0)) {
    local_54 = 0;
  }
  uVar12 = local_18;
  uVar14 = local_14;
  uVar15 = local_10;
  if (local_54 == 0) goto LAB_005183ae;
  if ((iVar10 == 0) || (bVar1 = false, (*(byte *)(iVar10 + 0x50) & 8) != 0)) {
    bVar1 = true;
  }
  uVar15 = *(uint *)(iVar8 + 0xac) & 1;
  uVar14 = *(uint *)(local_3c[5] + 0xac) & 1;
  bVar16 = ((byte)*(undefined4 *)(iVar9 + 0x24) & 0xc0) == 0x40;
  uVar12 = *(uint *)(local_24 + 0xac) & 1;
  local_44 = *(uint *)(local_20 + 0xac) & 1;
  uVar13 = *(uint *)(iVar9 + 0x50) >> 3 & 1;
  if (*(int *)(iVar7 + 0x60) == 0) {
    if (((uVar15 == 0) || (uVar14 == 0)) || (!bVar1)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    if (((uVar12 == 0) || (local_44 == 0)) || ((!bVar16 && (uVar13 == 0)))) {
LAB_0051831c:
      bVar4 = false;
    }
    else {
LAB_00518315:
      bVar4 = true;
    }
  }
  else if (*(int *)(iVar7 + 0x60) == 1) {
    bVar2 = bVar1;
    if (bVar16) goto LAB_00518315;
    if (uVar13 == 0) goto LAB_0051831c;
    bVar4 = true;
  }
  else {
    local_54 = 0;
    bVar2 = false;
  }
  if (local_54 != 0) {
    if ((bVar4) && (bVar2)) {
      if ((*(uint *)(iVar7 + 0x74) & 0x1000000) == 0) {
        if ((*(uint *)(iVar7 + 0x74) >> 0x14 & 1) == 0) {
          bVar4 = true;
          bVar2 = false;
        }
        else {
          bVar4 = false;
          bVar2 = true;
        }
      }
      else {
        bVar4 = true;
        bVar2 = false;
      }
    }
    bVar3 = bVar2;
    bVar5 = bVar4;
    if ((bVar4) || (bVar2)) {
      bVar6 = true;
    }
    else {
      bVar6 = false;
    }
  }
LAB_005183ae:
  uVar11 = local_1c;
  if (local_54 != 0) {
    uVar11 = 1;
    if (!bVar1) {
      uVar11 = local_1c;
    }
    if (uVar15 != 0) {
      uVar11 = uVar11 | 0x10;
    }
    if (uVar14 != 0) {
      uVar11 = uVar11 | 0x20;
    }
    if (bVar16) {
      uVar11 = uVar11 | 0x100;
    }
    if (uVar13 != 0) {
      uVar11 = uVar11 | 0x200;
    }
    if (uVar12 != 0) {
      uVar11 = uVar11 | 0x1000;
    }
    if (local_44 != 0) {
      uVar11 = uVar11 | 0x2000;
    }
    if (bVar3) {
      uVar11 = uVar11 | 0x100000;
    }
    if (bVar5) {
      uVar11 = uVar11 | 0x1000000;
    }
    if (bVar6) {
      uVar11 = uVar11 | 0x10000000;
    }
  }
  if ((local_54 != 0) && (*(uint *)(iVar7 + 0x74) != uVar11)) {
    *(uint *)(iVar7 + 0x74) = uVar11;
    FUN_004fd490((void *)(iVar7 + 0x194),param_1);
  }
  ExceptionList = local_c;
  return local_54;
}

