
void __fastcall FUN_005dd3a0(void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  void *this;
  undefined3 extraout_var_00;
  undefined4 uVar3;
  int iVar4;
  int *piVar5;
  int local_10;
  int local_c;
  int local_8;
  int local_4;
  
  iVar4 = 0;
  if (*(int **)((int)param_1 + 200) != (int *)0x0) {
    uVar3 = 0;
    local_10 = 0;
    local_8 = 0;
    local_4 = 0;
    local_c = 0;
    iVar2 = (**(code **)(**(int **)((int)param_1 + 200) + 4))();
    if (iVar2 == 1) {
      if (*(int *)((int)param_1 + 200) != 0) {
        iVar4 = *(int *)((int)param_1 + 200) + -0xd4;
      }
      local_8 = FUN_00596af0(iVar4);
      local_4 = FUN_00596b30(iVar4);
    }
    else {
      iVar4 = (**(code **)(**(int **)((int)param_1 + 200) + 4))();
      if ((iVar4 == 3) || (iVar4 = (**(code **)(**(int **)((int)param_1 + 200) + 4))(), iVar4 == 4))
      {
        iVar4 = *(int *)((int)param_1 + 200);
        local_4 = FUN_00596b60(iVar4);
        local_8 = FUN_00596b20(iVar4);
      }
    }
    piVar5 = (int *)((int)param_1 + 0x9c);
    iVar4 = 5;
    do {
      if (((void *)*piVar5 != (void *)0x0) &&
         (bVar1 = FUN_00603120((void *)*piVar5,4), CONCAT31(extraout_var,bVar1) == 1)) {
        uVar3 = FUN_00602510(*piVar5);
      }
      piVar5 = piVar5 + 1;
      iVar4 = iVar4 + -1;
    } while (iVar4 != 0);
    switch(uVar3) {
    case 0xdd10:
      local_10 = 7;
      break;
    case 0xdd11:
      local_10 = 8;
      break;
    case 0xdd12:
      local_10 = 9;
      break;
    case 0xdd13:
      local_10 = 10;
      break;
    case 0xdd14:
      local_10 = 0xb;
    }
    this = (void *)FUN_00604500((void *)((int)param_1 + 0x6c),0xdd20);
    if (this != (void *)0x0) {
      bVar1 = FUN_00603120(this,4);
      if (CONCAT31(extraout_var_00,bVar1) == 0) {
        FUN_00600c40(param_1,this,&DAT_006bd01c);
        local_c = 2;
      }
      else {
        FUN_00600c40(param_1,this,&DAT_006bcea8);
        local_c = 1;
      }
    }
    if (local_10 != local_8) {
      FUN_00595c00(local_10);
    }
    if (local_c != local_4) {
      FUN_00595c20(local_c);
    }
  }
  return;
}

