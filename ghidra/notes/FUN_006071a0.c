
void __thiscall FUN_006071a0(void *this)

{
  bool bVar1;
  undefined3 extraout_var;
  int in_stack_00000014;
  
  SetRectEmpty((LPRECT)((int)this + 0xbc));
  bVar1 = FUN_00607390(this,4);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    CopyRect((LPRECT)((int)this + 0xbc),(RECT *)&stack0x00000004);
    if (in_stack_00000014 != 0) {
      *(int *)((int)this + 0xcc) = in_stack_00000014;
    }
  }
  return;
}

