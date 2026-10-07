
void __thiscall
FUN_00458040(void *this,int param_1,undefined4 param_2,undefined4 param_3,int param_4)

{
  void *this_00;
  
  this_00 = *(void **)((int)this + param_1 * 0x38 + 0x19c);
  if ((*(int *)((int)this_00 + 0x94) != param_4) && (0 < param_4)) {
    FUN_0060e400(this_00,param_4,0);
  }
  FUN_0060e440(this_00,param_3);
  return;
}

