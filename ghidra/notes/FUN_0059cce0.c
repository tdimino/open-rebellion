
void __thiscall FUN_0059cce0(void *this,undefined4 param_1,int param_2)

{
  void *this_00;
  
  FUN_0059b600((int)this);
  this_00 = (void *)0x0;
  switch(param_1) {
  case 0:
    this_00 = *(void **)((int)this + 0xa8);
    break;
  case 1:
    this_00 = *(void **)((int)this + 0xac);
    break;
  case 2:
    this_00 = *(void **)((int)this + 0xb0);
    break;
  case 3:
    this_00 = *(void **)((int)this + 0xb4);
  }
  if ((this_00 != (void *)0x0) && (param_2 == 1)) {
    FUN_006030c0(this_00,4);
  }
  return;
}

