
void __thiscall FUN_00487eb0(void *this,undefined4 param_1)

{
  int *piVar1;
  int iVar2;
  
  *(undefined4 *)((int)this + 0x54) = param_1;
  switch(param_1) {
  case 1:
    iVar2 = 600;
    break;
  case 2:
    iVar2 = 0x3c;
    break;
  case 3:
    iVar2 = 0xc;
    break;
  case 4:
    iVar2 = 4;
    break;
  default:
    iVar2 = 0x3c;
    *(undefined4 *)((int)this + 0x54) = 1;
  }
  piVar1 = (int *)FUN_004f5cd0(0x280);
  if (piVar1 != (int *)0x0) {
    piVar1[8] = *(int *)((int)this + 0x24);
    piVar1[0xb] = iVar2;
    FUN_00487740(this,piVar1,0);
  }
  return;
}

