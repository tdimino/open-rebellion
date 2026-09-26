
uint FUN_00543040(int *param_1,uint *param_2,uint *param_3)

{
  bool bVar1;
  void *this;
  int iVar2;
  uint uVar3;
  uint *puVar4;
  uint *this_00;
  uint local_24;
  uint local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006471e8;
  local_c = ExceptionList;
  local_20[1] = 1;
  local_20[2] = -1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_004ece80(param_3);
  this = (void *)thunk_FUN_00506e60();
  FUN_004ece30(local_20);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_24);
  local_4 = CONCAT31(local_4._1_3_,2);
  iVar2 = 0;
  if (this != (void *)0x0) {
    iVar2 = FUN_005431e0(param_1,local_20);
  }
  if (iVar2 != 0) {
    iVar2 = FUN_00543360((int)param_1,&local_24);
    this_00 = param_3;
    if (iVar2 != 0) {
      uVar3 = FUN_005545e0(this,param_1,local_20,&local_24,param_2,1,param_3,(int *)(local_20 + 1));
      goto LAB_00543104;
    }
  }
  uVar3 = 0;
  this_00 = param_3;
LAB_00543104:
  if ((uVar3 != 0) && (local_20[2] != -1)) {
    FUN_004ece80(this_00);
  }
  puVar4 = FUN_0042d170(param_1,&param_3);
  local_20[3] = 0x65;
  local_20[4] = 0x66;
  local_4._0_1_ = 3;
  if ((*puVar4 >> 0x18 < 0x65) || (0x65 < *puVar4 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  if (bVar1) {
    uVar3 = 1;
    param_3 = (uint *)0x98000481;
    local_4._0_1_ = 4;
    FUN_004f26d0(this_00,&param_3);
    local_4._0_1_ = 2;
    FUN_00619730();
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar3;
}

