
bool __thiscall FUN_005892a0(void *this,int *param_1,undefined4 *param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  undefined3 extraout_var;
  bool bVar4;
  
  bVar4 = true;
  *param_2 = 0;
  iVar2 = FUN_004ece60((uint *)(param_1 + 0x27));
  if (iVar2 != 0) {
    piVar3 = (int *)FUN_00505190((uint *)(param_1 + 0x27));
    bVar4 = piVar3 != (int *)0x0;
    if (piVar3 != (int *)0x0) {
      bVar1 = FUN_00588d30(*(void **)((int)this + 4),param_1,piVar3,param_3);
      if ((CONCAT31(extraout_var,bVar1) != 0) && (bVar4)) {
        return true;
      }
      bVar4 = false;
    }
  }
  return bVar4;
}

