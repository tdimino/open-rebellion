
uint __thiscall FUN_00580b00(void *this,int *param_1)

{
  int *this_00;
  uint uVar1;
  void *this_01;
  uint *puVar2;
  uint local_68;
  undefined4 local_64;
  undefined4 local_60;
  int local_5c [2];
  uint local_54 [2];
  undefined4 local_4c [7];
  undefined4 local_30 [9];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_0064ea68;
  pvStack_c = ExceptionList;
  local_64 = 1;
  local_60 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_64);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_5c);
  local_4 = 1;
  FUN_0051fcb0(this,local_5c);
  uVar1 = FUN_005202d0(this,this_00);
  local_68 = 0;
  if ((uVar1 != 0) && (this_00[1] == -1)) {
    uVar1 = FUN_00553b80(*(int *)((int)this + 0x40),&local_68);
  }
  FUN_00551060(local_30);
  local_4._0_1_ = 2;
  if (uVar1 != 0) {
    if (this_00[1] == -1) {
      param_1 = (int *)0x3;
      uVar1 = FUN_00551190(local_30,local_5c,local_68,&param_1);
    }
    if (uVar1 != 0) {
      if (this_00[1] == -1) {
        uVar1 = FUN_005513a0(local_30,local_54);
      }
      if (uVar1 != 0) {
        if (this_00[1] == -1) {
          this_01 = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
          while (((this_01 != (void *)0x0 && (uVar1 != 0)) && (this_00[1] == -1))) {
            FUN_004ece30(&param_1);
            local_4._0_1_ = 3;
            FUN_0054fa40(local_4c);
            local_4._0_1_ = 4;
            puVar2 = FUN_004fa150(this_01,&local_64);
            local_4._0_1_ = 5;
            uVar1 = FUN_00551630(local_30,puVar2,local_54,this_00,local_4c,(uint *)&param_1);
            local_4 = CONCAT31(local_4._1_3_,4);
            FUN_00619730();
            if ((uVar1 != 0) && (this_00[1] == -1)) {
              FUN_004f26d0((void *)((int)this_01 + 0x48),&param_1);
            }
            local_4._0_1_ = 3;
            FUN_0054fa90(local_4c);
            local_4._0_1_ = 2;
            FUN_00619730();
            this_01 = *(void **)((int)this_01 + 0x10);
          }
        }
        if (uVar1 != 0) goto LAB_00580cf7;
      }
    }
  }
  local_64 = 1;
  local_60 = 1;
  local_4._0_1_ = 6;
  FUN_00520580(this_00,&local_64);
  local_4._0_1_ = 2;
  FUN_00619730();
LAB_00580cf7:
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005510b0(local_30);
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return uVar1;
}

