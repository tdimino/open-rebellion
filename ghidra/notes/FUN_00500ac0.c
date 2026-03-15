
void __thiscall
FUN_00500ac0(void *this,uint param_1,uint param_2,undefined4 param_3,undefined4 param_4,
            void *param_5)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006408d8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x17;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_5,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00553410((int)this,param_1,param_2,1,1,param_5);
  ExceptionList = local_c;
  return;
}

