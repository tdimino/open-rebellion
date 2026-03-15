
int FUN_005073d0(uint param_1,uint param_2,int param_3,uint *param_4,void *param_5)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  int iVar4;
  int iVar5;
  int local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00641630;
  local_c = ExceptionList;
  local_2c[4] = 1;
  local_2c[6] = 1;
  local_2c[7] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_5,local_2c + 6);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (param_3 == 0x204) {
    local_2c[6] = 8;
    local_2c[7] = 0x10;
    if ((*param_4 < 8) || (0x10 < param_4[1])) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) goto LAB_0050747b;
    bVar1 = true;
  }
  else {
LAB_0050747b:
    bVar1 = false;
  }
  if (param_3 == 0x204) {
    local_2c[6] = 0x10;
    local_2c[7] = 0x14;
    if ((*param_4 < 0x10) || (0x14 < param_4[1])) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) goto LAB_005074cb;
    local_2c[2] = 1;
  }
  else {
LAB_005074cb:
    local_2c[2] = 0;
  }
  if (param_3 == 0x240) {
    local_2c[6] = 0x30;
    local_2c[7] = 0x40;
    if ((*param_4 < 0x30) || (0x40 < param_4[1])) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) goto LAB_00507521;
    bVar2 = true;
  }
  else {
LAB_00507521:
    bVar2 = false;
  }
  if (param_3 == 0x241) {
    local_2c[6] = 0x30;
    local_2c[7] = 0x40;
    if ((*param_4 < 0x30) || (0x40 < param_4[1])) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      local_2c[6] = 1;
      goto LAB_00507579;
    }
  }
  local_2c[6] = 0;
LAB_00507579:
  iVar4 = local_2c[0];
  if ((((*(int *)((int)param_5 + 4) == -1) && ((*(byte *)(local_2c[0] + 0x50) & 8) != 0)) &&
      (!bVar1)) && (!bVar2)) {
    local_2c[1] = 0x22;
    local_2c[0] = 1;
    local_4 = 1;
    FUN_00520580(param_5,local_2c);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (((*(int *)((int)param_5 + 4) == -1) &&
      (local_2c[4] = FUN_00553410(iVar4,param_1,param_2,1,1,param_5), local_2c[4] != 0)) &&
     (*(int *)((int)param_5 + 4) != -1)) {
    if ((bVar1) || (bVar2)) {
      local_2c[0] = 1;
      local_2c[1] = 0xffffffff;
      local_4 = 2;
      FUN_00520580(param_5,local_2c);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if ((((local_2c[2] != 0) && ((*(uint *)(iVar4 + 0x88) & 2) != 0)) &&
        (((byte)*(undefined4 *)(iVar4 + 0x24) & 0xc0) == 0xc0)) &&
       ((*(uint *)(iVar4 + 0x88) & 1) == 0)) {
      local_2c[2] = 1;
      local_2c[3] = 0xffffffff;
      local_4 = 3;
      FUN_00520580(param_5,local_2c + 2);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  iVar5 = local_2c[4];
  if (((local_2c[4] != 0) && (*(int *)((int)param_5 + 4) == -1)) &&
     ((*(byte *)(iVar4 + 0x88) & 0x20) != 0)) {
    local_2c[4] = 0x90;
    local_2c[5] = 4;
    local_4 = 4;
    FUN_00520580(param_5,local_2c + 4);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (((bVar1) || (bVar2)) || ((local_2c[6] != 0 || (param_3 == 0x3f0)))) {
      local_2c[6] = 1;
      local_2c[7] = 0xffffffff;
      local_4 = 5;
      FUN_00520580(param_5,local_2c + 6);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return iVar5;
}

