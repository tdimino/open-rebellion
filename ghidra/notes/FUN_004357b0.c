
void * __thiscall FUN_004357b0(void *this,void *param_1)

{
  bool bVar1;
  void *this_00;
  undefined4 *puVar2;
  void *this_01;
  int iVar3;
  int iVar4;
  int local_2c;
  uint local_28;
  uint local_24;
  uint local_20;
  undefined1 local_1c [4];
  void *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d42f;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  local_18 = this;
  FUN_004ece30(&local_28);
  local_4 = 1;
  FUN_004ece30(&local_2c);
  local_4 = CONCAT31(local_4._1_3_,2);
  local_20 = 0;
  local_24 = 0;
  if (*(int *)((int)this + 8) == 0) {
    this_01 = (void *)0x0;
  }
  else {
    this_01 = (void *)(*(int *)((int)this + 8) + -0xc);
  }
  if (this_01 != (void *)0x0) {
    this_00 = (void *)FUN_0041c210((int)this_01);
    while (this_00 != (void *)0x0) {
      iVar4 = 1;
      puVar2 = FUN_0041c2c0(this_00,local_1c);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_2c,puVar2);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      local_24 = *(uint *)((int)this_00 + 0x14);
      if (*(int *)((int)this_01 + 0x14) == 0) {
        iVar3 = 0;
      }
      else {
        iVar3 = *(int *)((int)this_01 + 0x14) + -0xc;
      }
      while (iVar3 != 0) {
        if (iVar4 == 0) goto LAB_004358c0;
        iVar4 = FUN_00435980(&local_2c,(int *)&local_24,iVar3);
        if (*(int *)(iVar3 + 0x14) == 0) {
          iVar3 = 0;
        }
        else {
          iVar3 = *(int *)(iVar3 + 0x14) + -0xc;
        }
      }
      if (iVar4 == 0) {
LAB_004358c0:
        this_00 = *(void **)((int)this_00 + 8);
        puVar2 = FUN_0041c120(this_01,&local_2c);
        if (puVar2 != (undefined4 *)0x0) {
          (**(code **)*puVar2)(1);
        }
      }
      else {
        if (local_20 < local_24) {
          local_20 = local_24;
          FUN_004f26d0(&local_28,&local_2c);
        }
        this_00 = *(void **)((int)this_00 + 8);
      }
    }
  }
  local_18 = (void *)0x1;
  local_14 = 0xff;
  if ((local_28 >> 0x18 == 0) || (0xfe < local_28 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if ((bVar1) && (puVar2 = FUN_0041c120(this_01,(int *)&local_28), puVar2 != (undefined4 *)0x0)) {
    (**(code **)*puVar2)(1);
  }
  FUN_004f26d0(param_1,&local_28);
  local_10 = 1;
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

