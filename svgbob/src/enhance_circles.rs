use fragments::Block;
use fragments::Block::{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
};
use properties::Location;
use properties::PointBlock;
use patterns::FocusChar;
use fragments::Direction;

use fragments::Fragment;
use fragments::{
    line,
    arrow_line,
    start_arrow_line,
    arc,
    arc_major,
    open_circle,
    solid_circle,
};

use fragments::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};

pub trait Round{
    
    fn round(&self) -> (Vec<Fragment>, Vec<Location>, bool);
}


impl <'g>Round for FocusChar<'g>{
    ///
    /// Enhance drawings by making circular shapes into circle
    /// element
    ///  example:
    ///   _   .-.    .--.
    ///  (_) (   )  (    )
    ///       `-'    `--'
    ///  
    ///
    /// The circle is held by the element that is the center of the circle
    /// as oppused to arcs where the arcs
    /// are held by the element at its center along the arc points
    /// 
    /// Arc and circles will be treated differently
    fn round(&self) -> (Vec<Fragment>, Vec<Location>, bool) {
        let a = &PointBlock::block(A);
        let b = &PointBlock::block(B);
        let c = &PointBlock::block(C);
        let d = &PointBlock::block(D);
        let e = &PointBlock::block(E);
        let f = &PointBlock::block(F);
        let g = &PointBlock::block(G);
        let h = &PointBlock::block(H);
        let i = &PointBlock::block(I);
        let j = &PointBlock::block(J);
        let k = &PointBlock::block(K);
        let l = &PointBlock::block(L);
        let m = &PointBlock::block(M);
        let n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        let p = &PointBlock::block(P);
        let q = &PointBlock::block(Q);
        let r = &PointBlock::block(R);
        let s = &PointBlock::block(S);
        let t = &PointBlock::block(T);
        let u = &PointBlock::block(U);
        let v = &PointBlock::block(V);
        let w = &PointBlock::block(W);
        let x = &PointBlock::block(X);
        let y = &PointBlock::block(Y);
        
        let top = || Location::go(Top);
        let bottom = || Location::go(Bottom);
        let left = || Location::go(Left);
        let right = || Location::go(Right);
        let top_left = || Location::go(TopLeft);
        let top_right = || Location::go(TopRight);
        let bottom_left = || Location::go(BottomLeft);
        let bottom_right = || Location::go(BottomRight);


        let go_top = |step| Location::jump(Top,step);
        let go_bottom = |step| Location::jump(Bottom,step);
        let go_left = |step| Location::jump(Left,step);
        let go_right = |step| Location::jump(Right,step);
        let go_top_left = |step| Location::jump(TopLeft,step);
        let go_top_right = |step| Location::jump(TopRight,step);
        let go_bottom_left = |step| Location::jump(BottomLeft,step);
        let go_bottom_right = |step| Location::jump(BottomRight,step);

        let mut elm = vec![];
        let mut consumed = vec![];
        // tells whether that element
        // containing the arc/circle is along
        // with it, if along then no
        // additional processing is needed for this element
        // if not along, then further enhancement
        // intended behaviors would have to take effect
        let mut along_arc = false;

        let mut matched_arc = false;
        let mut matched_circle = false;
        let enable_round_pill = true;


        
        // circle 0
        //   _
        //  (_)
        if  self.is('_')
            && self.left().is('(')
            && self.right().is(')')
            && self.top().is('_'){

            elm.push(open_circle(m, 4));
            consumed.extend(vec![
                top(), left(), right(),
            ]);
            matched_circle = true;
            along_arc = true;// since the circle is too small
            // that the center and the bottom points of
            // the circle share the same element `_`
        }
        

        // circle 1
        {
            let mut quadrants = vec![];
            let mut matched_quadrant2 = false;
            let mut matched_quadrant1 = false;
            let mut matched_quadrant3 = false;
            let mut matched_quadrant4 = false;
            let mut matched_circle1 = false;
            //   .-
            //  ( +
            if self.top().is('-')
                && self.top().left().any(".,")
                && self.in_left(2).is('(') {
                
                quadrants.extend(vec![
                    arc(&top().k(), &go_left(2).y(), 8),
                ]);
                consumed.extend(vec![
                    top_left(),
                    go_left(2),
                ]);
                matched_arc = true;
                matched_quadrant2 = true;
            }
            //  -.
            //  + )
            if self.top().is('-')
                && self.top().right().any(".,")
                && self.in_right(2).is(')'){
                quadrants.extend(vec![
                    arc(&go_right(2).u(), &top().o(), 8),
                ]);
                consumed.extend(vec![
                    top_right(),
                    go_right(2),
                ]);
                matched_arc = true;
                matched_quadrant1 = true;
            }

            //  ( +
            //   `-
            if self.bottom().is('-')
                && self.bottom().left().any("`'")
                && self.in_left(2).is('(') {
                
                quadrants.extend(vec![
                    arc(&go_left(2).e(), &bottom().k(), 8),
                ]);
                consumed.extend(vec![
                    bottom_left(),
                    go_left(2),
                ]);
                matched_arc = true;
                matched_quadrant3= true;
            }

            //  + )
            //  -'
            if self.bottom().is('-')
                && self.bottom().right().any("`'")
                && self.in_right(2).is(')'){
                quadrants.extend(vec![
                    arc(&bottom().o(), &go_right(2).a(), 8),
                ]);
                consumed.extend(vec![
                    bottom_right(),
                    go_right(2),
                ]);
                matched_arc = true;
                matched_quadrant4= true;
            }
            

            // circle 1
            //  .-.
            // ( + )
            //  `-'
            if matched_quadrant1
                && matched_quadrant2
                && matched_quadrant3
                && matched_quadrant4{
                elm.push(open_circle(m, 8));
                consumed.extend(vec![
                    top(),
                    bottom(),
                    go_left(2),
                    go_right(2),
                    top_left(),
                    top_right(),
                    bottom_left(),
                    bottom_right(),
                ]);
                matched_circle = true;
                along_arc = false;
            }
        }
        

        // circle 2
        {
            let mut quadrants = vec![];
            let mut matched_quadrant1 = false;
            let mut matched_quadrant2 = false;
            let mut matched_quadrant3 = false;
            let mut matched_quadrant4 = false;

            //   .-
            //  ( +
            if self.top().is('-')
                && self.top().left().any(".,")
                && self.in_left(2).is('(') {
                
                quadrants.extend(vec![
                    arc(&top().k(), &go_left(2).y(), 8),
                ]);
                consumed.extend(vec![
                    top_left(),
                    go_left(2),
                ]);
                matched_arc = true;
                matched_quadrant2 = true;
            }

            //  ( +
            //   `-
            if self.bottom().is('-')
                && self.bottom().left().any("`'")
                && self.in_left(2).is('(') {
                
                quadrants.extend(vec![
                    arc(&go_left(2).e(), &bottom().k(), 8),
                ]);
                consumed.extend(vec![
                    bottom_left(),
                    go_left(2),
                ]);
                matched_arc = true;
                matched_quadrant3= true;
            }

            //  --.
            //  +  )
            if self.top().is('-')
                && self.top().right().is('-')
                && self.top().in_right(2).any(".,")
                && self.in_right(3).is(')'){
                quadrants.extend(vec![
                    arc(&go_right(3).u(), &top().right().o(), 8),
                ]);
                consumed.extend(vec![
                    top().go_right(2),
                    go_right(3),
                ]);
                matched_arc = true;
                matched_quadrant1 = true;
            }


            //  +  )
            //  --'
            if self.bottom().is('-')
                && self.bottom().right().is('-')
                && self.bottom().in_right(2).any("`'")
                && self.in_right(3).is(')'){
                quadrants.extend(vec![
                    arc(&bottom().right().o(), &go_right(3).a(), 8),
                ]);
                consumed.extend(vec![
                    bottom().go_right(2),
                    go_right(3),
                ]);
                matched_arc = true;
                matched_quadrant4= true;
            }

            // circle 2
            //  .--.
            // ( +  )
            //  `--'
            if matched_quadrant1
                && matched_quadrant2
                && matched_quadrant3
                && matched_quadrant4{
                elm.push(open_circle(o, 10));
                matched_circle = true;
                along_arc = false;

                consumed.push(right());// HACK:
                consumed.extend(vec![
                    top(),
                    top_left(),
                    go_left(2),
                    bottom_right(),
                    bottom(),
                    bottom_right(),
                    bottom().go_right(2),
                    go_right(3),
                    top().go_right(2),
                    top_right(),
                ]);
            }
        }


        /////////////////////////////
        //  top left arc of circle 4
        //     _.-
        //   .'  |
        //  (----+
        //
        /////////////////////////////
        {   // if 4 of them match then consume all, and make a full circle
            let mut quadrants = vec![];//temp storage for the arcs, replace with circle when all quadrants matched
            let mut matched_quadrant2 = false;
            let mut matched_quadrant1 = false;
            let mut matched_quadrant3 = false;
            let mut matched_quadrant4 = false;
            if self.in_left(5).is('(')
                && self.in_left(4).top().is('.')
                && self.in_left(3).top().is('\'')
                && self.in_left(2).in_top(2).is('_')
                && self.left().in_top(2).is('.')
                && self.in_top(2).is('-') 
            {
                quadrants.push(arc(&go_top(2).m(),
                            &go_left(4).m(),
                             18));
                matched_quadrant2 = true;
                consumed.extend(vec![
                    go_left(5),
                    go_left(4).top(),
                    go_left(3).top(),
                    go_left(2).go_top(2),
                    left().go_top(2),
                    go_top(2)
                ]);
                matched_arc = true;
                along_arc = false;
            }
            ///////////////////////////////
            // top right arc of the circle4
            //  -._
            //  |  `.
            //  +----)
            //////////////////////////////
            if self.in_right(5).is(')')
                && self.in_right(4).top().is('.')
                && self.in_right(3).top().any("`'")
                && self.in_right(2).in_top(2).is('_')
                && self.right().in_top(2).is('.')
                && self.in_top(2).is('-'){
                quadrants.push(
                    arc(&go_right(4).m(),
                        &go_top(2).m(),
                        18)
                );
                matched_quadrant1 = true;
                consumed.extend(vec![
                    go_right(5),
                    go_right(4).top(),
                    go_right(3).top(),
                    go_right(2).go_top(2),
                    right().go_top(2),
                    go_top(2)
                ]);
                matched_arc = true;
                along_arc = false;
            }
            ////////////////////////////////
            //  bottom_left arc of the circle4 
            //   
            //  (----+
            //   `._ |
            //      `-
            ////////////////////////////////
            if self.in_left(5).is('(')
                && self.in_left(4).bottom().any("`'")
                && self.in_left(3).bottom().is('.')
                && self.in_left(2).bottom().is('_')
                && self.left().in_bottom(2).any("`'")
                && self.in_bottom(2).is('-'){
                quadrants.push(
                    arc(&go_left(4).m(),
                        &go_bottom(2).m(),
                        18)
                );
                matched_quadrant3 = true;
                consumed.extend(vec![
                   go_left(5),
                   go_left(4).bottom(),
                   go_left(3).bottom(),
                   go_left(2).bottom(),
                   left().go_bottom(2),
                   go_bottom(2)
                ]);
                matched_arc = true;
                along_arc = false;
            }
            ///////////////////////////////////
            //  bottom_right arc of the circle4
            //    +----)
            //    | _,'
            //    -'
            //
            ////////////////////////////////////
            if self.in_right(5).is(')')
                && self.in_right(4).bottom().is('\'')
                && self.in_right(3).bottom().is(',')
                && self.in_right(2).bottom().is('_')
                && self.right().in_bottom(2).is('\'')
                && self.in_bottom(2).is('-'){
                quadrants.push(
                    arc(&go_bottom(2).m(),
                        &go_right(4).m(),
                        18)
                );
                matched_quadrant4 = true;
                consumed.extend(vec![
                    go_right(5),
                    go_right(4).bottom(),
                    go_right(3).bottom(),
                    go_right(2).bottom(),
                    right().go_bottom(2),
                    go_bottom(2),
                ]);
                matched_arc = true;
                along_arc = false;
            }
            if matched_quadrant2
                && matched_quadrant1
                && matched_quadrant3
                && matched_quadrant4{
                elm.push(open_circle(m, 18));
                matched_circle = true;
                along_arc = false;
            }
            else{
                elm.extend(quadrants);
            }
        }

        ////////////////////////////////
        //
        //  Circle 6
        //
        //////////////////////////////////
        {

            let mut quadrants = vec![];//temp storage for the arcs, replace with circle when all quadrants matched
            let mut matched_quadrant2 = false;
            let mut matched_quadrant1 = false;
            let mut matched_quadrant3 = false;
            let mut matched_quadrant4 = false;
            //        _         _ 
            //      ,'|       .'| 
            //     /  |      /  | 
            //    |---+     |---+  
            if self.in_top(3).is('_')
                && self.in_top(2).left().any("`'") 
                && self.in_top(2).in_left(2).any(".,") 
                && self.top().in_left(3).is('/') 
                && self.in_left(4).is('|'){

                    quadrants.extend(vec![
                                 arc(&go_top(3).u(),
                                     &go_left(4).c(), 20),
                                 line(&go_left(4).c(), &go_left(4).w()),
                              ]);
                    
                    consumed.extend(vec![
                                    go_top(2).left(), 
                                    go_top(2).go_left(2),
                                    top().go_left(3),
                                    go_left(4),
                        ]);
                    matched_arc = true;
                    along_arc = false;
                    matched_quadrant2 = true;
             }
             //     _
             //     |`.
             //     |  \
             //     +---|
             if self.in_top(3).is('_')
                 && self.in_top(2).right().any("`'") 
                 && self.in_top(2).in_right(2).any(".,")
                 && self.top().in_right(3).is('\\') 
                 && self.in_right(4).is('|'){

                 quadrants.extend(vec![
                         arc(&go_right(4).m(),
                             &go_top(3).y(),  20),
                         line(&go_right(4).c(),
                              &go_right(4).w())
                     ]); 
                 consumed.extend(vec![
                        go_top(2).right(),
                        go_top(2).go_right(2),
                        top().go_right(3),
                        go_right(4)
                 ]);
                 matched_arc = true;
                 along_arc = false;
                 matched_quadrant1 = true;
             }

            ////////////////////////////////
            //
            //   |---+
            //    \  |
            //     `._
            //
            ///////////////////////////////
            if self.in_left(4).is('|')
                && self.bottom().in_left(3).is('\\') 
                && self.in_bottom(2).in_left(2).any("`'") 
                && self.in_bottom(2).in_left(1).any(".,")
                && self.in_bottom(2).is('_'){

                quadrants.extend(vec![
                    arc(&go_left(4).m(), &go_bottom(2).u(), 20),
                    line(&go_left(4).m(), &go_left(4).c())
                ]);

                consumed.extend(vec![
                      go_left(4),
                      bottom().go_left(3),
                      go_bottom(2).go_left(2),
                      go_bottom(2).go_left(1),
                ]);
                matched_arc = true;
                along_arc = false;
                matched_quadrant3 = true;
            }
            /////////////////////////////////
            //
            //        +---|     +---|
            //        |  /      |  /
            //        _.'       _,'
            //
            ////////////////////////////////
            if self.in_right(4).is('|')
                && self.bottom().in_right(3).is('/')
                && self.in_bottom(2).in_right(2).any("`'") 
                && self.in_bottom(2).right().any(".,") 
                && self.in_bottom(2).is('_') {
                quadrants.extend(vec![
                        arc(&go_bottom(2).y(), &go_right(4).w(),  20),
                        line(&go_right(4).w(), &go_right(4).c())
                    ]);
                consumed.extend(vec![
                   go_right(4),
                   bottom().go_right(3),
                   go_bottom(2).go_right(2),
                   go_bottom(2).right(),
                ]);
                matched_arc = true;
                along_arc = false;
                matched_quadrant4 = true;
            }
            elm.extend(quadrants);
        }
        

        /*
        // if true
        //    .     .
        //   (  and  ) will be drawn from the top to bottom with 1 arc
        //    `     '
        // else the default behavior is used which is drawing with combination 
        // of rounded corner and lines connecting the arc at `(` or `)`
        if !matched_arc
            && !matched_circle{

            if self.is('('){
                if enable_round_pill{
                    //   .
                    //  (
                    //   `
                    if self.top_right().any(".,")
                        && self.bottom_right().any("`'"){
                        elm.extend(
                            vec![
                                arc(&top_right().o(), &bottom_right().o(), 10),
                            ]);
                        consumed.extend(vec![
                            top_right(), 
                            bottom_right()
                        ]);
                        along_arc = true;
                    }
                }
            }
            if self.is(')'){
                if enable_round_pill{
                    //   .
                    //    )
                    //   '
                    if self.top_left().any(".,")
                        && self.bottom_left().any("`'"){
                        elm.extend(
                            vec![
                                arc(&bottom_left().k(), &top_left().k(), 10),
                            ]);
                        consumed.extend(vec![
                            top_left(), 
                            bottom_left()
                        ]);
                        along_arc = true;
                    }
                }
            }
        }
        */
        (elm, consumed, along_arc)
    }
}

