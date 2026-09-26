
void __thiscall FUN_0042d190(void *this,int param_1,undefined4 param_2)

{
  int *piVar1;
  uint uVar2;
  void *pvVar3;
  
  if (*(int *)((int)this + 0x9c) == 1) {
    uVar2 = 0x20;
    do {
      FUN_00442590(uVar2,(void *)0x7,0,param_1,param_2,this,0x20000ff,0x2ffffff,0x2808080);
      uVar2 = uVar2 + 1;
    } while ((int)uVar2 < 0x25);
  }
  else {
    uVar2 = 0x25;
    do {
      FUN_00442590(uVar2,(void *)0x7,0,param_1,param_2,this,0x200ff00,0x2ffffff,0x2808080);
      uVar2 = uVar2 + 1;
    } while ((int)uVar2 < 0x2a);
  }
  FUN_005ff440((int)this);
  pvVar3 = this;
  piVar1 = (int *)FUN_006037f0(7);
  FUN_00442380(param_1,param_2,piVar1,(int)this,pvVar3);
  return;
}

