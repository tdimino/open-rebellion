
void __thiscall FUN_00612ce0(void *this,int *param_1,int param_2)

{
  byte bVar1;
  bool bVar2;
  int *this_00;
  int iVar3;
  
  iVar3 = param_2;
  this_00 = param_1;
  bVar2 = false;
  if (*(int *)(*(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 0x4c) != -1) {
    param_1 = (int *)0x0;
    FUN_00616cd0(this_00,param_2 + *(int *)((int)this + 0x1c),0);
    FUN_006173b0(this_00,&param_1,4);
    if (((((char)param_1 == 'd') && (param_1._1_1_ == 'a')) && (param_1._2_1_ == 't')) &&
       (param_1._3_1_ == 'a')) {
LAB_00612dea:
      FUN_006173b0(this_00,(int)this + 0x18,4);
      FUN_00616cd0(this_00,0xfffffff8,1);
      bVar2 = true;
    }
    else {
      param_2 = 0;
      *(undefined4 *)((int)this + 0x1c) = 0xc;
      FUN_00616cd0(this_00,iVar3 + 0xc,0);
      bVar1 = *(byte *)(*(int *)(*this_00 + 4) + 8 + (int)this_00);
      while ((bVar1 & 1) == 0) {
        FUN_006173b0(this_00,&param_1,4);
        if ((((char)param_1 == 'd') && (param_1._1_1_ == 'a')) &&
           ((param_1._2_1_ == 't' && (param_1._3_1_ == 'a')))) goto LAB_00612dea;
        FUN_006173b0(this_00,&param_2,4);
        FUN_00616cd0(this_00,param_2,1);
        *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + param_2 + 4;
        bVar1 = *(byte *)(*(int *)(*this_00 + 4) + 8 + (int)this_00);
      }
    }
  }
  if (!bVar2) {
    *(undefined4 *)((int)this + 0x1c) = 0;
    *(undefined4 *)((int)this + 0x18) = 0;
  }
  return;
}

