
undefined4 __cdecl FUN_005539f0(int param_1,void *param_2)

{
  int *this;
  undefined4 local_1c;
  undefined4 local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006490a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (int *)FUN_0041c210((int)param_2);
  while (this != (int *)0x0) {
    FUN_005f6250(param_2,(int)this);
    FUN_00541eb0(&local_1c,this + 0x10);
    local_18 = *(undefined4 *)(param_1 + 4);
    local_4 = 0;
    FUN_0054f3e0(this,&local_1c);
    FUN_0054f3f0(this);
    FUN_004fd370(this);
    local_4 = 0xffffffff;
    FUN_00541ea0();
    this = (int *)FUN_0041c210((int)param_2);
  }
  ExceptionList = local_c;
  return 1;
}

