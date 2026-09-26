
void __cdecl FUN_005c1160(uint param_1)

{
  undefined4 uVar1;
  void *this;
  
  this = (void *)FUN_005c1cd0(DAT_006bc700,param_1);
  if (this != (void *)0x0) {
    uVar1 = *(undefined4 *)((int)this + 0x40);
    FUN_005d3650(this,2);
    FUN_005d3650(this,1);
    FUN_005d3650(this,0);
    FUN_005d3650(this,uVar1);
  }
  return;
}

