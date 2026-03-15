
void __thiscall FUN_0048a590(void *this,int param_1)

{
  void *this_00;
  int *this_01;
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634edb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(100);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    this_01 = (int *)0x0;
  }
  else {
                    /* WARNING: Load size is inaccurate */
    this_01 = (int *)FUN_0048b2e0(this_00,*this,param_1);
  }
  local_4 = 0xffffffff;
  if (this_01[0x10] != 0) {
    uVar1 = FUN_0041d120();
    FUN_005f5600(this_01,uVar1);
    FUN_0048a060(this,this_01);
  }
  ExceptionList = local_c;
  return;
}

