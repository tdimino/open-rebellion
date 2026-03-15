
void __thiscall FUN_00518040(void *this,uint *param_1)

{
  bool bVar1;
  int iVar2;
  undefined4 uVar3;
  uint uVar4;
  int *piVar5;
  int *piVar6;
  
  iVar2 = FUN_004fd330();
  if (iVar2 != *(int *)((int)this + 0x78)) {
    uVar3 = FUN_004fd330();
    *(undefined4 *)((int)this + 0x78) = uVar3;
    *(undefined4 *)((int)this + 0x7c) = 0;
    *(undefined4 *)((int)this + 0x80) = 0;
    *(undefined4 *)((int)this + 0x84) = 0;
    *(undefined4 *)((int)this + 0x88) = 0;
  }
  piVar5 = (int *)0x0;
  uVar4 = *param_1 & 3;
  if (uVar4 == 2) {
    piVar5 = (int *)((int)this + 0x84);
  }
  else if (uVar4 == 1) {
    piVar5 = (int *)((int)this + 0x7c);
  }
  piVar6 = (int *)0x0;
  if (uVar4 == 2) {
    piVar6 = (int *)((int)this + 0x88);
  }
  else if (uVar4 == 1) {
    piVar6 = (int *)((int)this + 0x80);
  }
  bVar1 = true;
  if ((piVar5 != (int *)0x0) && (0 < *piVar5)) {
    bVar1 = false;
  }
  if ((piVar6 != (int *)0x0) && (0 < *piVar6)) {
    bVar1 = false;
  }
  if (bVar1) {
    if (piVar5 != (int *)0x0) {
      *piVar5 = *piVar5 + 1;
    }
    if (piVar6 != (int *)0x0) {
      *piVar6 = *piVar6 + 1;
    }
  }
  return;
}

