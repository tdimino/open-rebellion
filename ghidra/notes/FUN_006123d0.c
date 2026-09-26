
void __thiscall FUN_006123d0(void *this,LPCSTR param_1,int param_2,undefined4 param_3,int param_4)

{
  void *this_00;
  int *piVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656f5b;
  local_c = ExceptionList;
  if (param_1 != (LPCSTR)0x0) {
    ExceptionList = &local_c;
    this_00 = (void *)FUN_00618b70(0x5c);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = FUN_00616700(this_00,param_1,0xa1,DAT_0066e57c,1);
    }
    local_4 = 0xffffffff;
    if (piVar1 != (int *)0x0) {
      FUN_00612480(this,piVar1,param_2,param_3,param_4);
      if (*(int *)((int)this + 0x4c) == 0) {
        (*(code *)**(undefined4 **)(*(int *)(*piVar1 + 4) + (int)piVar1))(1);
      }
    }
  }
  ExceptionList = local_c;
  return;
}

