
void __thiscall
FUN_005a6ac0(void *this,int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5)

{
  void *this_00;
  undefined **local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00652878;
  local_c = ExceptionList;
  local_1c = param_2;
  local_18 = param_3;
  local_20 = &PTR_LAB_0066c2c0;
  local_14 = param_4;
  local_10 = param_5;
  local_4 = 0;
  if (param_1 == 0) {
    this_00 = *(void **)((int)this + 0xc);
  }
  else {
    this_00 = *(void **)((int)this + 8);
  }
  ExceptionList = &local_c;
  FUN_005c7c00(this_00,&local_20);
  ExceptionList = local_c;
  return;
}

