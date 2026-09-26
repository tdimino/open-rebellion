
int * FUN_0045cc10(int *param_1,int param_2,int param_3)

{
  POINT pt;
  bool bVar1;
  bool bVar2;
  int *piVar3;
  BOOL BVar4;
  undefined3 extraout_var;
  RECT RStack_10;
  
  bVar1 = false;
  piVar3 = (int *)(**(code **)(*param_1 + 8))();
  while ((piVar3 != (int *)0x0 && (!bVar1))) {
    RStack_10.left = piVar3[0x10];
    RStack_10.top = piVar3[0x11];
    RStack_10.right = piVar3[0x12];
    RStack_10.bottom = piVar3[0x13];
    pt.y = param_3;
    pt.x = param_2;
    BVar4 = PtInRect(&RStack_10,pt);
    bVar1 = false;
    if ((BVar4 != 0) &&
       (((bVar1 = true, piVar3[3] == 0x10000 && ((void *)piVar3[8] != (void *)0x0)) &&
        (bVar2 = FUN_005fca00((void *)piVar3[8],param_2 - RStack_10.left,param_3 - RStack_10.top),
        CONCAT31(extraout_var,bVar2) == 0)))) {
      bVar1 = false;
    }
    if (!bVar1) {
      piVar3 = (int *)(**(code **)(*piVar3 + 0xc))();
    }
  }
  return piVar3;
}

