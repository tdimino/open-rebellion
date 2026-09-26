
void __thiscall FUN_005df4f0(void *this,undefined4 param_1)

{
  switch(param_1) {
  case 0xdd01:
    FUN_00595c00(*(undefined4 *)((int)this + 0xc4));
  case 0xdd02:
    FUN_005ef340((int)this);
    FUN_005c5a50(*(void **)((int)this + 0x20),0,*(undefined4 *)((int)this + 0xc0));
    return;
  case 0xdd03:
  case 0xdd04:
  case 0xdd05:
  case 0xdd06:
    FUN_005df5f0(this,param_1);
    FUN_005df560((int)this);
  default:
    return;
  }
}

