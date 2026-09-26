
void __thiscall FUN_005dd080(void *this,int param_1)

{
  bool bVar1;
  void *this_00;
  undefined3 extraout_var;
  undefined *puVar2;
  
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),0xdd20);
  switch(param_1) {
  case 0xdd01:
    FUN_005dd3a0(this);
  case 0xdd02:
    FUN_005ef340((int)this);
    FUN_005c5a50(*(void **)((int)this + 0x20),0,*(undefined4 *)((int)this + 200));
    return;
  default:
    return;
  case 0xdd20:
    if (this_00 != (void *)0x0) {
      bVar1 = FUN_00603120(this_00,4);
      if (CONCAT31(extraout_var,bVar1) == 0) {
        puVar2 = &DAT_006bd01c;
      }
      else {
        puVar2 = &DAT_006bcea8;
      }
      FUN_00600c40(this,this_00,puVar2);
    }
  case 0xdd10:
  case 0xdd11:
  case 0xdd12:
  case 0xdd13:
  case 0xdd14:
    FUN_005dd140(this,param_1);
    return;
  }
}

