
void __thiscall FUN_0059b130(void *this,int param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *this_00;
  undefined3 extraout_var_01;
  
  switch(param_1) {
  default:
    return;
  case 0xad02:
  case 0xad03:
  case 0xad04:
  case 0xad05:
  case 0xad06:
  case 0xad07:
  case 0xad08:
  case 0xad09:
    FUN_0059b450(this,param_1);
    return;
  case 0xad0b:
  case 0xad0c:
  case 0xad0d:
  case 0xad0e:
    FUN_0059b620(this,param_1);
    return;
  case 0xad10:
    FUN_00595b10(0xf);
    return;
  case 0xad12:
    FUN_00595b10(0x10);
    return;
  case 0xad13:
    FUN_00595b10(0x11);
    return;
  case 0xad14:
    FUN_00595b10(0x12);
    return;
  case 0xad15:
    FUN_00595b10(0x13);
    return;
  case 0xad16:
    FUN_00595b10(0x14);
    bVar1 = FUN_00603120(*(void **)((int)this + 0xa0),4);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      FUN_00600c40(this,*(void **)((int)this + 0xa0),&DAT_006bc330);
      return;
    }
    FUN_00600c40(this,*(void **)((int)this + 0xa0),&DAT_006bc27c);
    return;
  case 0xad17:
    FUN_00595b10(0x15);
    bVar1 = FUN_00603120(*(void **)((int)this + 0xa4),4);
    if (CONCAT31(extraout_var_00,bVar1) == 0) {
      FUN_00600c40(this,*(void **)((int)this + 0xa4),&DAT_006bc280);
      return;
    }
    FUN_00600c40(this,*(void **)((int)this + 0xa4),&DAT_006bc3d8);
    return;
  case 0xad18:
    FUN_0059b710(this,0x1b,0);
    return;
  case 0xad19:
    FUN_005c2990(*(void **)((int)this + 0x108),8,0);
    return;
  case 0xad1a:
    FUN_005c2990(*(void **)((int)this + 0x108),7,0);
    return;
  case 0xad1b:
    FUN_005c2990(*(void **)((int)this + 0x108),1,0);
    return;
  case 0xad1c:
    FUN_005c2990(*(void **)((int)this + 0x108),2,0);
    return;
  case 0xad1d:
    FUN_005c2990(*(void **)((int)this + 0x108),3,0);
    return;
  case 0xad1e:
    FUN_005c2990(*(void **)((int)this + 0x108),4,0);
    return;
  case 0xad1f:
    FUN_005c2990(*(void **)((int)this + 0x108),5,0);
    return;
  case 0xad20:
    FUN_005c2990(*(void **)((int)this + 0x108),6,0);
    return;
  case 0xad21:
    FUN_005c2990(*(void **)((int)this + 0x108),9,0);
    return;
  case 0xad22:
    break;
  }
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),0xad22);
  bVar1 = FUN_00596a50();
  if (CONCAT31(extraout_var_01,bVar1) == 1) {
    FUN_00596a30();
    if (this_00 == (void *)0x0) {
      return;
    }
    FUN_006030f0(this_00,4);
    FUN_00600c40(this,this_00,&DAT_006bc29c);
    return;
  }
  FUN_00596a10();
  if (this_00 == (void *)0x0) {
    return;
  }
  FUN_006030c0(this_00,4);
  FUN_00600c40(this,this_00,&DAT_006bc298);
  return;
}

