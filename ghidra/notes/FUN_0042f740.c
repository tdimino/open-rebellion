
void __thiscall FUN_0042f740(void *this,uint *param_1)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  void *this_00;
  
  iVar2 = FUN_0042f700(this,(int *)param_1);
  if (iVar2 == 0) {
    if ((*param_1 >> 0x18 < 0x50) || (0x7f < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      piVar3 = FUN_004f3000(*(int *)((int)this + 0xc),param_1);
      if (((piVar3 != (int *)0x0) && ((*(byte *)(piVar3 + 0x29) & 4) == 0)) && (piVar3[0x16] != 0))
      {
        this_00 = (void *)FUN_005f5500(this,piVar3[0x16]);
        if (this_00 != (void *)0x0) {
          FUN_0047adf0(this_00,param_1);
        }
      }
    }
  }
  return;
}

