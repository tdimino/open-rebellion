
void __thiscall FUN_00499460(void *this,int param_1,uint *param_2,int param_3)

{
  int *piVar1;
  uint uVar2;
  int *piVar3;
  undefined4 *puVar4;
  int iVar5;
  uint local_4;
  
  iVar5 = 0;
  local_4 = 0;
  if (param_1 != 0x14b) {
    return;
  }
  piVar1 = FUN_004f3220(*(int *)((int)this + 0x20),param_2);
  FUN_004f26d0((void *)((int)this + 0x5c),param_2);
  if (piVar1 != (int *)0x0) {
    uVar2 = (uint)piVar1[9] >> 6 & 3;
    if (uVar2 == 1) {
      iVar5 = FUN_004f3dd0(*(int *)((int)this + 0x20),1);
      *(undefined2 *)((int)this + 0x2e) = 0x3f3;
      local_4 = (uint)(*(int *)((int)this + 0x20) == 1);
    }
    else if (uVar2 == 2) {
      iVar5 = FUN_004f3dd0(*(int *)((int)this + 0x20),2);
      *(undefined2 *)((int)this + 0x2e) = 0x3f4;
      local_4 = (uint)(*(int *)((int)this + 0x20) == 2);
    }
    else {
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    if ((*(byte *)(piVar1 + 0x22) & 4) == 0) {
      if (param_3 == 0) {
        *(undefined4 *)((int)this + 0x40) = 1;
        goto LAB_0049952c;
      }
    }
    else if (param_3 != 0) {
      *(undefined4 *)((int)this + 0x40) = 1;
      goto LAB_0049952c;
    }
  }
  *(undefined4 *)((int)this + 0x40) = 0;
LAB_0049952c:
  if (iVar5 == 0) {
    *(undefined4 *)((int)this + 0x40) = 0;
  }
  if (*(int *)((int)this + 0x40) != 0) {
    if (param_3 == 0) {
      *(undefined4 *)((int)this + 100) = 0;
      *(undefined2 *)((int)this + 0x30) = 0x455;
      if (piVar1 == (int *)0x0) {
        piVar3 = (int *)0x0;
      }
      else {
        piVar3 = piVar1 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7031,piVar3,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (piVar1 == (int *)0x0) {
        piVar1 = (int *)0x0;
      }
      else {
        piVar1 = piVar1 + 0xc;
      }
      if (iVar5 == 0) {
        puVar4 = (undefined4 *)0x0;
      }
      else {
        puVar4 = (undefined4 *)(iVar5 + 0x30);
      }
      FUN_0060b9d0((void *)((int)this + 0x50),0x7032,puVar4,piVar1,(undefined4 *)0x0,
                   (undefined4 *)0x0);
    }
    else {
      *(undefined4 *)((int)this + 100) = 1;
      *(undefined2 *)((int)this + 0x30) = 0x454;
      if (piVar1 == (int *)0x0) {
        piVar3 = (int *)0x0;
      }
      else {
        piVar3 = piVar1 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7030,piVar3,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (piVar1 == (int *)0x0) {
        piVar1 = (int *)0x0;
      }
      else {
        piVar1 = piVar1 + 0xc;
      }
      if (iVar5 == 0) {
        puVar4 = (undefined4 *)0x0;
      }
      else {
        puVar4 = (undefined4 *)(iVar5 + 0x30);
      }
      FUN_0060b9d0((void *)((int)this + 0x50),0x7033,puVar4,piVar1,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      *(undefined2 *)((int)this + 0x2e) = 0x3f2;
    }
    if (local_4 != *(uint *)((int)this + 100)) {
      *(undefined4 *)((int)this + 0x28) = 1;
      return;
    }
    *(undefined4 *)((int)this + 0x28) = 2;
  }
  return;
}

