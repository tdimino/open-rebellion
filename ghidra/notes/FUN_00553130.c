
int __thiscall FUN_00553130(void *this,uint *param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  int *piVar2;
  int iVar3;
  
  *param_2 = 0;
  if (*(int *)((int)this + 0x10) == -1) {
    if (*(int *)((int)this + 0x14) == 1) {
      iVar3 = 1;
      puVar1 = (undefined4 *)0x1;
    }
    else if (*(int *)((int)this + 0x14) == 2) {
      iVar3 = 1;
      puVar1 = (undefined4 *)&DAT_00000002;
    }
    else {
      iVar3 = 0;
      puVar1 = (undefined4 *)0x0;
    }
  }
  else {
    iVar3 = 0;
    puVar1 = param_2;
  }
  if (iVar3 != 0) {
    piVar2 = FUN_004f2d10((int)puVar1,param_1);
    *param_2 = piVar2;
  }
  return iVar3;
}

