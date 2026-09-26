
uint __cdecl FUN_00518be0(int *param_1,uint *param_2,void *param_3)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  uint *puVar5;
  uint uVar6;
  int iVar7;
  uint *puVar8;
  undefined4 *puVar9;
  uint local_18;
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar8 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642b8f;
  local_c = ExceptionList;
  uVar6 = *param_2;
  local_18 = 1;
  if ((((int)uVar6 < 1) || (3 < (int)uVar6)) && (uVar6 != 0)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  uVar6 = param_2[3];
  if ((((int)uVar6 < 1) || (3 < (int)uVar6)) || (uVar6 == 3)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  if (((int)param_2[5] < 1) || (3 < (int)param_2[5])) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  if ((((bVar2) && (bVar3)) && (bVar4)) && (param_2[6] != 0)) {
    ExceptionList = &local_c;
    puVar5 = FUN_0051bf10(param_2,local_14);
    local_4 = 0;
    uVar6 = (**(code **)(*param_1 + 4))();
    if ((uVar6 < *puVar5) || (puVar5[1] <= uVar6)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((bVar2) && ((((uint)param_1[9] >> 6 & 3) == *puVar8 || (*puVar8 == 0)))) {
      uVar6 = puVar8[6];
      if (puVar8[5] != 1) {
        uVar6 = -uVar6;
      }
      if ((((param_1[0x22] & 1U) != 0) && ((param_1[0x22] & 8U) == 0)) &&
         (uVar1 = puVar8[4], iVar7 = FUN_00507270(param_1,puVar8[3]), iVar7 < (int)uVar1)) {
        puVar8 = (uint *)FUN_00618b70(0x44);
        if (puVar8 == (uint *)0x0) {
          FUN_00619730();
        }
        local_4 = 1;
        param_2 = puVar8;
        if (puVar8 == (uint *)0x0) {
          puVar9 = (undefined4 *)0x0;
        }
        else {
          puVar9 = FUN_00562c70(puVar8);
        }
        local_18 = (uint)(puVar9 != (undefined4 *)0x0);
        local_4 = 0xffffffff;
        if ((local_18 != 0) && (local_18 = puVar9[6], local_18 == 0)) {
          if (puVar9 == (undefined4 *)0x0) {
            ExceptionList = local_c;
            return 0;
          }
          (**(code **)*puVar9)(1);
        }
        if (puVar9 != (undefined4 *)0x0) {
          puVar8 = FUN_004025b0(param_1,(uint *)&param_2);
          local_4 = 2;
          FUN_004f26d0(puVar9 + 0xf,puVar8);
          local_4 = 0xffffffff;
          FUN_00619730();
          puVar9[0x10] = uVar6;
          FUN_004fd490(puVar9 + 8,param_3);
          FUN_0053fcd0(puVar9);
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_18;
}

