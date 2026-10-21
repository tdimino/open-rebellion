
void __thiscall FUN_00603010(void *this,undefined4 param_1,undefined4 param_2)

{
  switch(param_1) {
  case 0:
    *(undefined4 *)((int)this + 0xb4) = param_2;
    return;
  case 1:
    *(undefined4 *)((int)this + 0xb8) = param_2;
    return;
  case 2:
  case 0x40:
    *(undefined4 *)((int)this + 0xc0) = param_2;
    break;
  case 4:
    *(undefined4 *)((int)this + 0xbc) = param_2;
    return;
  }
  return;
}

